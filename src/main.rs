fn main() {
    for day in 1..13 {
        print_iteration(day);
    }
}

fn print_present(day: u8, gift: &str) {
    if day == 1 {
        println!("a {}.", gift);
        return;
    }
    println!("{} {},", day, gift);
}

fn find_present(day: u8) -> &'static str {
    // Match is the equivalent to a switch statement in this context.
    match day {
        1 => "partridge in a pear tree",
        2 => "turtle doves",
        3 => "french hens",
        4 => "calling birds",
        5 => "golden rings",
        6 => "gease-a-laying",
        7 => "swans-a-swimming",
        8 => "maids-a-milking",
        9 => "ladies dancing",
        10 => "lords-a-leaping",
        11 => "pipers piping",
        _ => "drummers drumming", // _ is a catch-all (similar to *)
    }
}

fn find_ordinal(day: u8) -> &'static str {
    match day {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eigth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        _ => "twelth",
    }
}

fn print_iteration(day: u8) {
    println!(
        "On the {} day of Christmas, my true love gave to me...",
        find_ordinal(day)
    );
    let mut index = day;
    while index > 0 {
        if day > 1 && index == 1 {
            print!("And ");
        }
        print_present(index, find_present(index));
        index -= 1;
    }
}
