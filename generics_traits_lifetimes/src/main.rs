fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Clone,
{
    let clone = list.to_vec();
    let mut largest = clone[0].clone();

    for item in clone {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() < s2.len() {
        s2
    } else {
        s1
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn ok(&self, some_other: &str) -> &str {
        println!("{}", some_other);
        self.part
    }
}

fn main() {
    // generic functions and traits
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // lifetimes

    let string1 = "abcd".to_string();
    let string2 = "xyz".to_string();

    let result = longest(&string1, &string2);
    println!("The longest string is {}", result);

    // lifetimes with structs
    let mut imp = ImportantExcerpt { part: "hello" };
    let result = imp.ok("world");
    println!("{}", result);

    //static lifetime
    let mut s: &'static str = "I have a static lifetime.";
    println!("{}", s);
    s = "";
    println!("{}", s);
}
