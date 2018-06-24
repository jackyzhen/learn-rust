mod stats;

mod pig_latin;

use pig_latin::pig_latin;
use stats::num_stats;

fn main() {
    let stats = num_stats(&mut vec![10, 2, 38, 23, 38, 23, 21]);
    println!("{:#?}", stats);

    let pig = pig_latin("Hello World Apple");
    println!("{}", pig);
}
