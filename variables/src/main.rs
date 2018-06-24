const F_RATIO: f64 = 9.0 / 5.0;
const C_RATIO: f64 = 5.0 / 9.0;

const NUMBERS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];
const PRESENTS: [&str; 12] = [
    "Partridge in a Pear Tree",
    "Turtle Doves",
    "French Hens",
    "Calling Birds",
    "Golden Rings",
    "Geese a Laying",
    "Swans a Swimming",
    "Maids a Milking",
    "Ladies Dancing",
    "Lords a Leaping",
    "Pipers Piping",
    "Drummers Drumming",
];

fn convert_temp(v: f64, is_c: bool) -> f64 {
    match is_c {
        true => F_RATIO * v + 32.0,
        false => C_RATIO * (v - 32.0),
    }
}

fn fib(nth: u32) -> u32 {
    match nth {
        0 => 0,
        1 => 1,
        _ => fib(nth - 1) + fib(nth - 2),
    }
}

fn christmas_song() {
    for days in 1..=12 {
        println!(
            "On the {} day of christmas,\nMy true love sent to me:",
            NUMBERS[days - 1]
        );
        for presents in (0..days).rev() {
            match (presents, days) {
                (0, 1) => println!("A {}", PRESENTS[0]),
                (0, _) => println!("and a {}", PRESENTS[0]),
                (p, _) => println!("{} {}", p + 1, PRESENTS[p]),
            }
        }
        println!("")
    }
}

fn main() {
    println!("40C = {}F", convert_temp(40.0, true));
    println!("156F = {}c", convert_temp(156.0, false));

    println!("fib 20 = {}", fib(20));
    christmas_song();
}
