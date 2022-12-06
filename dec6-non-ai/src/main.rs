use std::collections::HashSet;


fn part1(input:&str) -> usize {
    let chars:Vec<char> = input.chars().collect();
    for i in 0..chars.len() {
        if i > 2 {
            let uniques: HashSet<char> = HashSet::from_iter(chars[i-3..=i].iter().cloned());
            if uniques.len() == 4 {
                return i+1
            }
        }
    }
    panic!("not found");
}

fn part2(input:&str) -> usize {
    let chars:Vec<char> = input.chars().collect();
    for i in 0..chars.len() {
        if i > 12 {
            let uniques: HashSet<char> = HashSet::from_iter(chars[i-13..=i].iter().cloned());
            if uniques.len() == 14 {
                return i+1
            }
        }
    }
    panic!("not found");
}



#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_sample1() {
        assert_eq!(part1(include_str!("../input.txt")), 7);
    }

    #[test]
    fn test_sample2() {
        assert_eq!(part2(include_str!("../input.txt")), 19);
    }
}

fn main() {
    println!("part 1: {}", part1(include_str!("../input-full.txt")));
    println!("part 2: {}", part2(include_str!("../input-full.txt")));

}
