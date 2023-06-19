use advent_of_code::aoc_2015;

fn main() {
    let mut line = String::new();
    println!("Retrieve the answers to Advent of Code questions");
    println!("Enter the year and question number separated by a space (eg. 2015 1)");
    std::io::stdin().read_line(&mut line).unwrap();
    line.pop(); //remove trailing \n
    let year_question: Vec<&str> = line.split(" ").collect();
    let year_question: (&str, &str) = (year_question[0], year_question[1]);

    match year_question {
        ("2015", "1") => println!(
            "2015 1 part 1:\n\tThe instructions take Santa to floor {}.\n\tThe character with position {} took Santa to the basement for the first time",
            aoc_2015::aoc_2015_1::part_1(),
            aoc_2015::aoc_2015_1::part_2()
        ),
        _ => println!("Not implemented"),
    }
}
