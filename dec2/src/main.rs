fn main() {

    // win = 6, tie = 3, loss = 0
    // rock = 2, paper = 1, scissors = 3
    // A = rock, b = paper, c = scissors
    // x = rock, y = paper, z = scissors

    let score = include_str!("../input-full.txt")
        .lines()
        .fold(0, |acc,line|{
            let (opponent, my) = line.split_once(' ').unwrap();
            
            let win_score = match (opponent, my) {
                ("A", "X") => 3,
                ("A", "Y") => 6,
                ("A", "Z") => 0,
                ("B", "X") => 0,
                ("B", "Y") => 3,
                ("B", "Z") => 6,
                ("C", "X") => 6,
                ("C", "Y") => 0,
                ("C", "Z") => 3,
                _ => unreachable!()
            };

            let round = win_score + match my {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => unreachable!()
            };
            
            acc + round
        });

        println!("Score {}", score);

        // win = 6, tie = 3, loss = 0
        // rock = 2, paper = 1, scissors = 3
        // A = rock, b = paper, c = scissors
        // X = lose, Y = draw, = Z = win

        let score2 = include_str!("../input-full.txt")
        .lines()
        .fold(0, |acc,line|{
            let (opponent, next) = line.split_once(' ').unwrap();
            
            let (next_move, win_score) = match (opponent, next) {
                ("A", "X") => ("C", 0),
                ("A", "Y") => ("A", 3),
                ("A", "Z") => ("B", 6),
                ("B", "X") => ("A", 0),
                ("B", "Y") => ("B", 3),
                ("B", "Z") => ("C", 6),
                ("C", "X") => ("B", 0),
                ("C", "Y") => ("C", 3),
                ("C", "Z") => ("A", 6),
                _ => unreachable!()
            };

            let round = win_score + match next_move {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => unreachable!()
            };
            
            acc + round
        });

        println!("Score 2 {}", score2);


}
