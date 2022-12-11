use std::collections::VecDeque;

struct Monkey {
    items: VecDeque<u64>,
    operator: String,
    right_operand: Option<u64>,
    divisible_by: u64,
    monkey_on_true: usize,
    monkey_on_false: usize
}


fn parse(input: &str) -> Vec<Monkey> {

    let sv:Vec<&str> = input.lines().collect();
    let monkeys = sv.chunks(7).map(|c| {
        let items: VecDeque<u64>= c[1][18..].split(", ").map(|num_str| num_str.parse::<u64>().unwrap()).collect();
        let operator = c[2][23..24].to_string();

        let right_operand = match &c[2][25..] {
            "old" => None,
            x => Some(x.parse::<u64>().unwrap()),
        };
        let divisible_by = c[3][21..].parse::<u64>().unwrap();
        let monkey_on_true = c[4][29..].parse::<usize>().unwrap();
        let monkey_on_false = c[5][30..].parse::<usize>().unwrap();
        
        Monkey { items, operator, right_operand, divisible_by, monkey_on_true, monkey_on_false }
    }).collect();

    monkeys
}


fn part1(input: &str) -> u64 {
    println!("Part2");
    let mut m = parse(input);
    let mut inspections = vec![0;m.len()];
    for _rounds in 1..21 {
        for i in 0..m.len() {
            while let Some(item) = m[i].items.pop_front() {
                inspections[i] += 1;
                let new_item_worry = match (m[i].operator.as_str(), m[i].right_operand) {
                    ("+", None) => item + item,
                    ("*", None) => item * item,
                    ("+", Some(val)) => item + val,
                    ("*", Some(val)) => item * val,
                    _ => unreachable!(),
                };

                let reduced_worry = new_item_worry / 3;
                let monkey_index = if reduced_worry % m[i].divisible_by == 0 {
                    m[i].monkey_on_true
                } else {
                    m[i].monkey_on_false
                };
    
                m[monkey_index].items.push_back(reduced_worry);
            }
        }
    }

    inspections.sort();
    inspections.reverse();
    return inspections[0] * inspections[1];
}

fn part2(input: &str) -> u64 {
    println!("Part1");
    let mut m = parse(input);
    let mut inspections = vec![0;m.len()];
    let prim_product = m.iter().fold(1, |acc, x| acc * x.divisible_by);
    for _rounds in 1..10001 {
        for i in 0..m.len() {
            while let Some(item) = m[i].items.pop_front() {
                inspections[i] += 1;
                let new_item_worry = match (m[i].operator.as_str(), m[i].right_operand) {
                    ("+", None) => item + item,
                    ("*", None) => item * item,
                    ("+", Some(val)) => item + val,
                    ("*", Some(val)) => item * val,
                    _ => unreachable!(),
                };

                let monkey_index = if new_item_worry % m[i].divisible_by == 0 {
                    m[i].monkey_on_true
                } else {
                    m[i].monkey_on_false
                };
    
                m[monkey_index].items.push_back(new_item_worry % prim_product);
            }
        }
    }

    inspections.sort();
    inspections.reverse();
    return inspections[0] * inspections[1];
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_sample1() {
        assert_eq!(part1(include_str!("../input.txt")), 10605);
    }

    #[test]
    fn test_sample2() {
        assert_eq!(part2(include_str!("../input.txt")), 2713310158);
    }
}
fn main() {
    println!("part 1: {}", part1(include_str!("../input-full.txt")));
    println!("part 2: {}", part2(include_str!("../input-full.txt")));

}