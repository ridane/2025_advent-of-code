mod helpers;

use helpers::{
    generate_double_ids, generate_multi_repeat_ids, load_input, parse_input,
    sum_invalid_ids_in_ranges,
};

// Partie 1 : somme des IDs invalides formes d'une sequence repetee exactement deux fois
fn solve_part1(input: &str) -> u128 {
    let ranges = parse_input(input);

    if ranges.is_empty() {
        return 0;
    }

    let max_end = ranges.iter().map(|&(_, end)| end).max().unwrap();
    let invalid_ids = generate_double_ids(max_end);

    sum_invalid_ids_in_ranges(&invalid_ids, &ranges)
}

// Partie 2 : somme des IDs invalides formes d'une sequence repetee au moins deux fois
fn solve_part2(input: &str) -> u128 {
    let ranges = parse_input(input);

    if ranges.is_empty() {
        return 0;
    }

    let max_end = ranges.iter().map(|&(_, end)| end).max().unwrap();
    let invalid_ids = generate_multi_repeat_ids(max_end);

    sum_invalid_ids_in_ranges(&invalid_ids, &ranges)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = load_input()?;

    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 1_227_775_554);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 4_174_379_265);
    }
}
