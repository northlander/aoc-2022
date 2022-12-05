use std::collections::VecDeque;

use regex::Regex;

struct Move {
    num: usize,
    from: usize,
    to: usize,
}

fn parse_move(input: &str) -> Move {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let caps = re.captures(input).unwrap();
    
    Move { 
        num: caps.get(1).map_or(0, |m| m.as_str().parse::<usize>().unwrap()), 
        from: caps.get(2).map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
        to: caps.get(3).map_or(0, |m| m.as_str().parse::<usize>().unwrap())
    }
}

fn parse(input: &str) -> (usize, Vec<VecDeque<char>>, Vec<Move>) {
    let mut num_elements = 0;
    let mut index_line_index = 0;

    for (i, e) in input.lines().enumerate() {
        if e.starts_with(" 1") {
            let num_elements_str:String = e.chars().skip(e.len() - 2).take(1).collect();
            num_elements = num_elements_str.parse().unwrap();
            index_line_index = i;
            break;
        }
    }

    let mut stacks: Vec<VecDeque<char>> = Vec::with_capacity(num_elements);
    for _ in 0..num_elements {
        stacks.push(VecDeque::new());
    }
    let mut moves: Vec<Move> = Vec::new();

    for (i, e) in input.lines().enumerate() {
        if i < index_line_index { // Stack line
            for (j, c) in e.chars().enumerate() {
                if j % 4 == 1 { // at potential LETTER
                    let stack_index = j / 4;
                    if c.is_alphabetic() {
                        stacks.get_mut(stack_index).unwrap().push_back(c);
                    }
                }
            }
        } else if i > index_line_index + 1 { // Move line
            moves.push(parse_move(e));
        }
    }
    (num_elements, stacks, moves)
}

fn main() {    

    let input = include_str!("../input-full.txt");
    let (num, mut stacks, moves) = parse(input);


    // Execute all moves
    moves.iter().for_each(|m| {
       for _ in (0..m.num) {
            let popped = stacks.get_mut(m.from-1).unwrap().pop_front().unwrap();
            stacks.get_mut(m.to-1).unwrap().push_front(popped);
       }
    });
    
    println!("Part 1");
    for (i, e) in stacks.iter().enumerate() {
        let c = e.front().unwrap();
        println!("Stack {} -> {}", i, c);
    }


    let (num2, mut stacks2, moves2) = parse(input);


    // Execute all moves 2
    moves2.iter().for_each(|m| {

        let mut temp : VecDeque<char> = VecDeque::new();

       for _ in (0..m.num) {
            let popped = stacks2.get_mut(m.from-1).unwrap().pop_front().unwrap();
            temp.push_front(popped);
       }

       let target_stack = stacks2.get_mut(m.to-1).unwrap();
       
       temp.iter().for_each(|e| {
            target_stack.push_front(*e);
       });
    });
    
    println!("Part 2");
    for (i, e) in stacks2.iter().enumerate() {
        let c = e.front().unwrap();
        println!("Stack {} -> {}", i, c);
    }


}
