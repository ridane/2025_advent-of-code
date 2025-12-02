pub fn generate_double_ids(max_end: u64) -> Vec<u64> {
    let max_digits = count_digits(max_end);
    let mut ids = Vec::new();

    for half_len in 1..=max_digits / 2 {
        let pow10 = 10u64.pow(half_len);
        let start_base = pow10 / 10; // premier nombre a `half_len` chiffres

        for base in start_base..pow10 {
            let candidate = base * pow10 + base; // on repete la sequence deux fois

            if candidate > max_end {
                break; // les suivants ne feront qu'augmenter.
            }

            ids.push(candidate);
        }
    }

    ids.sort_unstable();
    ids.dedup();
    ids
}

pub fn generate_multi_repeat_ids(max_end: u64) -> Vec<u64> {
    let max_digits = count_digits(max_end);
    let mut ids = Vec::new();

    for len in 1..=max_digits {
        let max_repeats = max_digits / len;
        if max_repeats < 2 {
            continue; // impossible de repeter au moins deux fois avec cette longueur
        }

        let pow10 = 10u64.pow(len);
        let start_base = pow10 / 10; // premier nombre a `len` chiffres

        for base in start_base..pow10 {
            let mut candidate = base; // une repetition
            for _repeats in 2..=max_repeats {
                candidate = candidate * pow10 + base;

                if candidate > max_end {
                    break;
                }

                ids.push(candidate);
            }
        }
    }

    ids.sort_unstable();
    ids.dedup();
    ids
}

pub fn sum_invalid_ids_in_ranges(invalid_ids: &[u64], ranges: &[(u64, u64)]) -> u128 {
    let mut total = 0u128;

    for &(start, end) in ranges {
        let start_idx = invalid_ids.partition_point(|&id| id < start);
        let end_idx = invalid_ids.partition_point(|&id| id <= end);

        for &id in &invalid_ids[start_idx..end_idx] {
            total += id as u128;
        }
    }

    total
}

pub fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .split(|c| c == ',' || c == '\n')
        .filter(|raw| !raw.trim().is_empty())
        .map(|raw| {
            let (start, end) = raw
                .trim()
                .split_once('-')
                .expect("expected format: start-end");
            let start: u64 = start.parse().expect("invalid range start");
            let end: u64 = end.parse().expect("invalid range end");
            (start, end)
        })
        .collect()
}

pub fn load_input() -> Result<String, Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/day02.txt".to_string());

    Ok(std::fs::read_to_string(path)?)
}

fn count_digits(mut value: u64) -> u32 {
    let mut digits = 1;

    while value >= 10 {
        digits += 1;
        value /= 10;
    }
    digits
}
