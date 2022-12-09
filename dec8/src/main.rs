use std::collections::HashSet;

use vecgrid::Vecgrid;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l
                .as_bytes()
                .iter()
                .map(|b| b - 48)
                .collect()
                
        })
        .collect()
}

fn parse2(input: &str) -> Vecgrid<u8> { // part 2 - attempt to use vecgrid. Not sure if it really helped
    Vecgrid::from_rows(parse(input).clone()).unwrap()
}

fn part1(input:&str) -> i32 { // this can be done WAY better.
    let data = parse(input);

    let mut count = 0;
    let mut picked_trees: HashSet::<(usize, usize)> = HashSet::new();

    // left to right
    for y in 0..data.len() {
        let mut largest = 0;
        for x in 0..data[0].len() {
            if data[y][x] > largest || x == 0 {
                largest = data[y][x];
                if !picked_trees.contains(&(y,x)) {
                    count += 1;
                    picked_trees.insert((y,x));    
                }

                println!("1. Row {} Col {}", y, x);
            }
        }
    }

    // right to left
    for y in 0..data.len() {
        let mut largest = 0;
        for x in (0..data[0].len()).rev() {
            if data[y][x] > largest || x == data[0].len()-1 {
                largest = data[y][x];
                if !picked_trees.contains(&(y,x)) {
                    count += 1;
                    picked_trees.insert((y,x));    
                }
               
                println!("2. Row {} Col {}", y, x);
            }
        }
    }

    // top to bottom
    for x in 0..data[0].len() {
        let mut largest = 0;
        for y in 0..data.len() {
            if data[y][x] > largest || y == 0 {
                largest = data[y][x];
                if !picked_trees.contains(&(y,x)) {
                    count += 1;
                    picked_trees.insert((y,x));    
                }
                println!("3. Row {} Col {}", y, x);
            }
        }
    }

    // bottom to top
    for x in 0..data[0].len() {
        let mut largest = 0;
        for y in (0..data.len()).rev() {
            if data[y][x] > largest || y == data.len()-1 {
                largest = data[y][x];
                if !picked_trees.contains(&(y,x)) {
                    count += 1;
                    picked_trees.insert((y,x));    
                }
                println!("4. Row {} Col {}", y, x);
            }
        }
    }

    println!("{}", picked_trees.len());

    return count;
}

fn range(d: &[u8]) -> i32 {
    let mut count = 0;
    for i in 1..d.len() {
        count += 1;
        if i > 0 && d[i] >= d[0] {
            break;
        }
    }

    count
}

fn part2(input:&str) -> i32 {

    let data = parse2(input);
    let height = data.row_len();
    let width = data.column_len();
    let mut best = 0;

    for row in 0..data.num_rows() {
        for col in 0..data.num_columns() {

            // up
            let upslice = &mut data.as_columns()[col].clone()[0..row+1];
            upslice.reverse();
            let up = range(upslice);

            // down
            let downslice = &mut data.as_columns()[col].clone()[row..height];
            let down = range(downslice);

            let rightslice = &mut data.as_rows()[row].clone()[col..width];
            let right = range(rightslice);

            let leftslice = &mut data.as_rows()[row].clone()[0..col+1];
            leftslice.reverse();
            let left = range(leftslice);

            let sum = left * right * up * down;
            //println!("Sum Row {} Col {} = {}", row, col, sum);
            if sum > best {
                best = sum;
            }
        }
    }

    best
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_sample1() {
        assert_eq!(part1(include_str!("../input.txt")), 21);
    }

    #[test]
    fn test_sample2() {
        assert_eq!(part2(include_str!("../input.txt")), 8);
    }
}

fn main() {
    println!("part 1: {}", part1(include_str!("../input-full.txt")));
    println!("part 2: {}", part2(include_str!("../input-full.txt")));

}