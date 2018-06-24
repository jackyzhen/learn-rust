fn main() {
    let s1 = give_ownership();
    let s2 = s1;
    let _s3 = takes_and_gives_ownership(s2);
}

fn give_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

fn takes_and_gives_ownership(s: String) -> String {
    s
}
