mod database;
mod types;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("usage: dict <word>");
        return;
    }
    if args[1].is_ascii() {
        use wana_kana::ConvertJapanese;
        database::find_word(&args[1].to_hiragana()).unwrap();
    } else {
        database::find_word(&args[1]).unwrap();
    }
}
