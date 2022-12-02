fn main() {

    println!("Part one");

    let mut result :Vec<i32> = include_str!("../input1.txt")
        .split("\n\n")
        .map(|e| {
            e.lines().fold(0, |s, line| {
                s + line.parse::<i32>().unwrap()
            })
        })
        .collect();

    result.sort();
    result.reverse();

    let part2 = result[0..3]
        .to_vec()
        .into_iter()
        .reduce(|a, b| a + b);

    println!("{}", part2.unwrap());


}
