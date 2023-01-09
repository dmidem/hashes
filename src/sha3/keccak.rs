// See https://keccak.team/keccak_specs_summary.html for details of the algorithm.

// For ρ and π steps
const T: [(usize, usize, u32); 24] = [
    (0, 2, 1),
    (2, 1, 3),
    (1, 2, 6),
    (2, 3, 10),
    (3, 3, 15),
    (3, 0, 21),
    (0, 1, 28),
    (1, 3, 36),
    (3, 1, 45),
    (1, 4, 55),
    (4, 4, 2),
    (4, 0, 14),
    (0, 3, 27),
    (3, 4, 41),
    (4, 3, 56),
    (3, 2, 8),
    (2, 2, 25),
    (2, 0, 43),
    (0, 4, 62),
    (4, 2, 18),
    (2, 4, 39),
    (4, 1, 61),
    (1, 1, 20),
    (1, 0, 44),
];

const R: [u64; 24] = [
    0x0000000000000001,
    0x0000000000008082,
    0x800000000000808a,
    0x8000000080008000,
    0x000000000000808b,
    0x0000000080000001,
    0x8000000080008081,
    0x8000000000008009,
    0x000000000000008a,
    0x0000000000000088,
    0x0000000080008009,
    0x000000008000000a,
    0x000000008000808b,
    0x800000000000008b,
    0x8000000000008089,
    0x8000000000008003,
    0x8000000000008002,
    0x8000000000000080,
    0x000000000000800a,
    0x800000008000000a,
    0x8000000080008081,
    0x8000000000008080,
    0x0000000080000001,
    0x8000000080008008,
];

fn keccak_f1600_on_lanes(mut lanes: [[u64; 5]; 5]) -> [[u64; 5]; 5] {
    for r in R {
        // θ step
        {
            let c = lanes.map(|lane| lane[0] ^ lane[1] ^ lane[2] ^ lane[3] ^ lane[4]);

            let d = [(4, 1), (0, 2), (1, 3), (2, 4), (3, 0)]
                .map(|(i1, i2)| c[i1] ^ c[i2].rotate_left(1));

            for x in 0..5 {
                lanes[x].iter_mut().for_each(|a| *a ^= d[x]);
            }
        }

        // ρ and π steps
        {
            let mut a = lanes[1][0];
            for (x, y, r) in T {
                (a, lanes[x][y]) = (lanes[x][y], a.rotate_left(r));
            }
        }

        // χ step
        {
            for j in 0..5 {
                let t = [(1, 2), (2, 3), (3, 4), (4, 0), (0, 1)]
                    .map(|(i1, i2)| !lanes[i1][j] & lanes[i2][j]);
                (0..5).for_each(|i| lanes[i][j] ^= t[i])
            }
        }

        // ι step
        lanes[0][0] ^= r;
    }

    lanes
}

#[inline(always)]
fn into_lanes(state: [u8; 200]) -> [[u64; 5]; 5] {
    let mut lanes = [[0u64; 5]; 5];

    let mut i = 0;
    for y in 0..5 {
        for lane in &mut lanes {
            let j = i + 8;
            lane[y] = u64::from_le_bytes(state[i..j].try_into().unwrap());
            i = j;
        }
    }

    lanes
}

#[inline(always)]
fn from_lanes(lanes: [[u64; 5]; 5]) -> [u8; 200] {
    let mut state = [0u8; 200];

    let mut i = 0;
    for y in 0..5 {
        for lane in &lanes {
            let j = i + 8;
            state[i..j].copy_from_slice(&u64::to_le_bytes(lane[y]));
            i = j;
        }
    }

    state
}

#[inline(always)]
fn keccak_f1600(state: [u8; 200]) -> [u8; 200] {
    from_lanes(keccak_f1600_on_lanes(into_lanes(state)))
}

pub(crate) fn keccak<const OUTPUT_BYTE_LEN: usize>(
    rate_in_bytes: usize,
    input_bytes: &[u8],
    delimited_suffix: u8,
) -> Option<[u8; OUTPUT_BYTE_LEN]> {
    if rate_in_bytes == 0 || rate_in_bytes > 200 {
        return None;
    }

    // Absorb all the input blocks
    let mut last_block_size = 0;
    let mut lanes = [[0u64; 5]; 5];
    input_bytes.chunks(rate_in_bytes).for_each(|block| {
        (0..5)
            .flat_map(move |y| (0..5).map(move |x| (x, y)))
            .zip(block.chunks(8).map(|bytes| {
                let mut padded_bytes = [0u8; 8];
                padded_bytes[0..bytes.len()].copy_from_slice(bytes);
                u64::from_le_bytes(padded_bytes)
            }))
            .for_each(|((x, y), a)| {
                lanes[x][y] ^= a;
            });

        last_block_size = match block.len() {
            n if n == rate_in_bytes => {
                lanes = keccak_f1600_on_lanes(lanes);
                0
            }
            n => n,
        }
    });
    let mut state = from_lanes(lanes);

    // Do the padding and switch to the squeezing phase
    state[last_block_size] ^= delimited_suffix;
    if (delimited_suffix & 0x80) != 0 && last_block_size == (rate_in_bytes - 1) {
        state = keccak_f1600(state);
    }
    state[rate_in_bytes - 1] ^= 0x80;

    // Squeeze out all the output blocks
    let mut output_bytes = [0u8; OUTPUT_BYTE_LEN];
    output_bytes.chunks_mut(rate_in_bytes).for_each(|block| {
        state = keccak_f1600(state);
        block.copy_from_slice(&state[0..block.len()]);
    });

    Some(output_bytes)
}
