use std::collections::HashMap;
use std::env;
use std::hash::Hash;
use std::marker::PhantomData;
use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("bad input");
        process::exit(1);
    }

    let simulated_user_specified_value: u32 = if let Ok(i) = args[1].clone().parse() {
        i
    } else {
        eprintln!("failed to parse input {}", args[1]);
        process::exit(1);
    };

    let simulated_random_number: u32 = if let Ok(i) = args[2].clone().parse() {
        i
    } else {
        eprintln!("failed to parse input {}", args[2]);
        process::exit(1);
    };

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cacher<T, U, V>
where
    U: Clone,
    V: Clone,
    T: Fn(U) -> V,
{
    calculation: T,
    map: HashMap<U, V>,
    phantom: PhantomData<U>,
}

impl<T, U, V> Cacher<T, U, V>
where
    U: Clone + Hash + Eq,
    V: Clone,
    T: Fn(U) -> V,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            phantom: PhantomData,
            map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        let calculation = &self.calculation;
        self.map
            .entry(arg.clone())
            .or_insert_with(|| (calculation)(arg))
            .clone()
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_calculation = Cacher::new(|num| {
        println!("Calculating Slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_calculation.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_calculation.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_calculation.value(intensity)
            );
        }
    }
}
