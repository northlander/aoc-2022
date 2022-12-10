fn compile(input: &str) -> Vec<Option<i32>> {
    let mut compiled:Vec<Option<i32>> = Vec::new();
    input.lines().for_each(|l|{
        if l == "noop" {
            compiled.push(None);
        } else {
            compiled.push(None); // emit extra none to simulate two cycles
            compiled.push(Some(l[5..].parse::<i32>().unwrap()));
        }
    });
    compiled
}

fn check_cycle(cycle: i32) -> bool {
    match cycle {
        _ if cycle == 20 => true,
        _ if cycle >= 60 => (cycle-20) % 40 == 0,
        _ => false,
    }
}

fn should_draw(pos: i32, reg: i32) -> bool {
    (pos - reg).abs() < 2
}


fn part1(input: &str) -> i32 {
    let (_, _, final_sum) = compile(input)
        .iter()
        .fold((1,1,0), |(cycle, register_x, sum), line| {
            match line {
                None if check_cycle(cycle) => (cycle + 1, register_x, sum + cycle * register_x),
                None => (cycle + 1, register_x, sum),
                Some(op) if check_cycle(cycle) => (cycle + 1, register_x + op, sum + cycle * register_x),
                Some(op) => (cycle + 1, register_x + op, sum),
            }
        });
    final_sum
}


fn part2(input: &str) {
    let mut reg = 1;
    let mut cycle = 1;
    for op in compile(input) {
        reg = match op {
            None => reg,
            Some(o) => reg + o
        };

        if should_draw(cycle, reg) {
            print!("#");
        } else {
            print!("-");
        }
        if cycle % 40 == 0 {
            println!("!");
            cycle = 1;
        } else {
            cycle += 1;
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_sample1() {
        assert_eq!(part1(include_str!("../input.txt")), 13140);
    }

    #[test]
    fn test_part2(){
        part2(include_str!("../input.txt"));
        assert_eq!(true, true);
    }
}
fn main() {
    println!("part 1: {}", part1(include_str!("../input_full.txt")));
    part2(include_str!("../input_full.txt"));
}