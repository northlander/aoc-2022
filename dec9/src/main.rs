use std::{str::FromStr, collections::HashSet};


#[derive(Debug, PartialEq, Clone, Copy)]
enum Dir { 
    Right,
    Up,
    Left,
    Down, 
}

impl FromStr for Dir {
    type Err = ();
    fn from_str(input: &str) -> Result<Dir, Self::Err> {
        match input {
            "U" => Ok(Dir::Up),
            "D" => Ok(Dir::Down),
            "L" => Ok(Dir::Left),
            "R" => Ok(Dir::Right),
            _ => Err(()),
        }
    }
}

fn parse(input: &str) -> Vec<(Dir,i32)>{
    input
        .lines()
        .map(|l| {
            let (dir, distance) = l.split_at(1);
            (Dir::from_str(dir).unwrap(), distance[1..].parse::<i32>().unwrap())
        })
        .collect()
}

fn update_head((state_x, state_y): (i32,i32), dir: Dir) -> (i32, i32) {
    match dir {
        Dir::Up => (state_x, state_y - 1),
        Dir::Down => (state_x, state_y + 1),
        Dir::Right => (state_x + 1, state_y),
        Dir::Left => (state_x -1, state_y),
    }
}

fn update_tail( (tail_x, tail_y): (i32,i32), (head_x, head_y): (i32, i32), dir: Dir) -> (i32, i32) {
    if (head_x - tail_x).abs() > 1 || (head_y - tail_y).abs() > 1 {
        match dir {
            Dir::Up => (head_x, head_y + 1),
            Dir::Down => (head_x, head_y - 1),
            Dir::Right => (head_x - 1, head_y),
            Dir::Left => (head_x + 1, head_y),
        }
    } else {
        (tail_x, tail_y)
    }
}

fn update_tail2( (tail_x, tail_y): (i32,i32), (head_x, head_y): (i32, i32)) -> (i32, i32) {
    if (head_x - tail_x).abs() > 1 || (head_y - tail_y).abs() > 1 {
        let x_diff = head_x - tail_x;
        let y_diff = head_y - tail_y;
        let x_move = if x_diff <= -1 { -1 } else if x_diff >= 1 { 1 } else { 0 };
        let y_move = if y_diff <= -1 { -1 } else if y_diff >= 1 { 1 } else { 0 };
        (tail_x + x_move, tail_y + y_move)
    } else {
        (tail_x, tail_y)
    }
}

fn part1(input: &str) -> usize {
    
    let moves = parse(input);
    let mut head_state = (0,0);
    let mut tail_state = (0,0);

    let mut history:HashSet<(i32,i32)> = HashSet::new();

    for (dir, num) in moves {
        for _ in 0..num {
            head_state = update_head(head_state, dir);
            tail_state = update_tail(tail_state, head_state, dir);
            history.insert(tail_state);
        }
    }

    history.len()
}

fn printme(history:&HashSet<(i32,i32)>){
  println!("");
  for y in -15..15 {
    let s:String = (-15..15).enumerate().map(|(x, _)| {
        if history.contains(&(x as i32, y as i32)) { 
            "#" 
        } else { 
            "." 
        }
    }).collect();
    println!("{}", s);
  }
  println!("");
}

fn printme2(state:&Vec<(i32,i32)>){
    println!("");
    for y in -10..5 {
        let s:String = (-10..10).map(|x| {
            let mut ch = ".".to_string();
            for (i, (sx,sy)) in state.iter().enumerate() {
                if x as i32 == (*sx as i32) && y == (*sy as i32) {
                    ch = i.to_string();
                }
            }
            ch
        }).collect();
        println!("{}", s);
    }
    println!("");
}

fn part2(input: &str) -> usize {
    let moves = parse(input);
    let mut tail_states = vec![(0,0);10];
    let mut history:HashSet<(i32,i32)> = HashSet::new();
    history.insert((0,0));
    for (dir, num) in moves {
        for _ in 0..num {
            tail_states[0] = update_head(tail_states[0], dir);
            for i in 1..10 {
                tail_states[i] = update_tail2(tail_states[i], tail_states[i-1]);
            }
            history.insert(tail_states[9]);
           // printme2(&tail_states);
        }
    }

    history.len()
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_sample1() {
        assert_eq!(part1(include_str!("../input.txt")), 13);
    }

    #[test]
    fn test_sample2() {
        assert_eq!(part2(include_str!("../input2.txt")), 36);
    }
}
fn main() {
    println!("part 1: {}", part1(include_str!("../input-full.txt")));
    println!("part 2: {}", part2(include_str!("../input-full.txt")));

}