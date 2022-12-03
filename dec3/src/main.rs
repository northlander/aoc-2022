fn prio(c: char) -> u32 {
    if c.is_ascii_uppercase() {
       c as u32 - 38
     } else {
        c as u32 - 96
     }
}

fn main() {
    let total = include_str!("../input-full.txt")
        .lines()
        .fold(0, |s, l| {
            let (left, right) = l.split_at(l.len()/2);
            let equal_char: Vec<char> = left
                .chars()
                .into_iter()
                .filter(|c| right.contains(*c))
                .collect();
           
            s + prio(equal_char[0])
        });

    println!("total part 1 {}", total);

    let lines = include_str!("../input-full.txt")
        .lines().collect::<Vec<&str>>();

    let mut total2 = 0;
    for i in (0..lines.len() - 1).step_by(3) {
        println!("{}", i);

        let equal_char: Vec<char> = lines[i]
            .chars()
            .into_iter()
            .filter(|c| {
                lines[i+1].contains(*c) 
                && lines[i+2].contains(*c) 
            })
            .collect();
        total2 = total2 + prio(equal_char[0]);
    }
    println!("total part 2 {}", total2);

}
