use std::ops::Add;

fn add_thing<T: Add>(fst: T, snd: T) {
    let _ = fst + snd;
}

fn show_me<T: display>(val: T) {
    // can use {} format string now, because of Display bound
    println!("{}", val);
}

fn main() {
    add_thing(2, 2);
}
