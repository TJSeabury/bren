use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let word: &String = &args[1];
        let upper: String = word.to_uppercase();
        println!("{:?}", upper);
    }
}
