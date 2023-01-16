mod database;
mod types;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        // TODO: terminal UI
        println!("usage: dict <word>");
        return;
    }
    let reverse = args.len() == 3 && args[2] == "-";
    if args[1].is_ascii() {
        use wana_kana::ConvertJapanese;
        database::find_word(&args[1].to_hiragana(), reverse).unwrap();
    } else {
        database::find_word(&args[1], reverse).unwrap();
    }
}
