mod database;
mod types;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("usage: dict <word>");
        return;
    }
    database::find_word(&args[1]).unwrap();
}
