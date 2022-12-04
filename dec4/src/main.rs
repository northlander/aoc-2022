use std::ops::Range;


fn to_range(t: &str) -> Range<u32> {
    let (start, end) = t.split_once('-').unwrap();
    start.parse::<u32>().unwrap()..(end.parse::<u32>().unwrap()+1) // ranges end are exclusive, must add to cover!!
}

fn main() {

    // part 1
    let total = include_str!("../input-full.txt")
        .lines()
        .fold(0, |s, l|{
            let (elf1, elf2) = l.split_once(',').unwrap();
            let r1 = to_range(elf1);
            let r2 = to_range(elf2);

            if r1.len() < r2.len() {

                if r1.fold(true, |acc, val| acc && r2.contains(&val) ) {
                    return s + 1
                }
            } else {
                if r2.fold(true, |acc, val| acc && r1.contains(&val) ) {
                    return s + 1
                }
            }

            s
        });

    println!("Total1: {}", total);


    // part 2

    let total2 = include_str!("../input-full.txt")
        .lines()
        .fold(0, |s, l|{
            let (elf1, elf2) = l.split_once(',').unwrap();
            let r1 = to_range(elf1);
            let r2 = to_range(elf2);

            if r1.len() < r2.len() {

                if r1.fold(false, |acc, val| acc || r2.contains(&val) ) {
                    return s + 1
                }
            } else {
                if r2.fold(false, |acc, val| acc || r1.contains(&val) ) {
                    return s + 1
                }
            }

            s
        });

    println!("Total1: {}", total2);


}
