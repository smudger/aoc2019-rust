pub fn part1(input: Option<&str>) -> String {
    input
        .unwrap_or(include_str!("../input.txt"))
        .lines()
        .map(|num| (num.parse::<usize>().unwrap() / 3) - 2)
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: Option<&str>) -> String {
    input
        .unwrap_or(include_str!("../input.txt"))
        .lines()
        .map(|num| {
            let mut fuel = 0;
            let mut mass = num.parse::<isize>().unwrap();
            loop {
                mass = (mass / 3) - 2;
                if mass <= 0 {
                    break;
                }
                fuel += mass;
            }

            fuel
        })
        .sum::<isize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_works() {
        assert_eq!(
            part1(Some(include_str!("../example01.txt"))),
            "34241".to_string()
        );
    }

    #[test]
    fn part_2_works() {
        assert_eq!(
            part2(Some(include_str!("../example01.txt"))),
            "51316".to_string()
        );
    }
}
