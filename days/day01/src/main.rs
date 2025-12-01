const START_POS: i32 = 50;
const RING_SIZE: i32 = 100;


// Partie 2 : on compte chaque passage par 0 pendant un mouvement (y compris si on boucle plusieurs fois).
fn solve_part2(input: &str) -> i32 {
    let moves = parse_input(input);
    let mut pos = START_POS;
    let mut hits = 0;

    for delta in moves {
        let steps = delta.abs(); // on prend la distance absolue pour que le comptage des passages par 0 marche dans les deux sens.

        if steps > 0 {
            let offset = if delta > 0 { // rotation a droite : distance jusqu'au prochain zero en bouclant sur POSITIONS.
                if pos == 0 {
                    RING_SIZE
                } else {
                    RING_SIZE - pos
                }

            } else { // rotation a gauche : distance jusqu'au zero precedent en bouclant sur POSITIONS.
                if pos == 0 {
                    RING_SIZE
                } else {
                    pos
                }
            };

            if steps >= offset {
                hits += 1 + (steps - offset) / RING_SIZE;
            }
        }

        pos = (pos + delta).rem_euclid(RING_SIZE);
    }

    hits
}

// Partie 1 : on compte uniquement si la position finale d'un mouvement tombe sur 0.
fn solve_part1(input: &str) -> i64 {
    let moves = parse_input(input);
    let mut pos = START_POS;
    let mut hits = 0;

    for delta in moves {

        pos = (pos + delta).rem_euclid(RING_SIZE); // on ramene la position dans l'anneau.

        if pos == 0 { // on compte les passages exacts sur 0
            hits += 1;
        }
    }

    hits
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (dir, amount) = line.split_at(1);
            let distance: i32 = amount.parse().expect("distance should be a number");
            match dir {
                "L" => -distance,
                "R" => distance,
                _ => panic!("direction should be L or R"),
            }
        })
        .collect()
}


fn load_input() -> Result<String, Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/day/01.txt".to_string());

    Ok(std::fs::read_to_string(path)?)
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
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 6);
    }

    #[test]
    fn part2_large_rotation() {
        // Depuis 50, R1000 depasse le zero 10 fois.
        assert_eq!(solve_part2("R1000\n"), 10);
    }
}
