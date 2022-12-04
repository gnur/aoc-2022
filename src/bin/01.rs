pub fn part_one(input: &str) -> Option<u32> {
    let mut i = 0;
    let mut elves = vec![0];

    for line in input.lines() {
        if line == "" {
            i += 1;
            elves.push(0);
            continue;
        }
        let num: u32 = line.parse().unwrap();
        elves[i] += num;
    }

    elves.sort();

    Some(elves[i])
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut i = 0;
    let mut elves = vec![0];

    for line in input.lines() {
        if line == "" {
            i += 1;
            elves.push(0);
            continue;
        }
        let num: u32 = line.parse().unwrap();
        elves[i] += num;
    }

    elves.sort();

    let sum = elves[i] + elves[i - 1] + elves[i - 2];

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
