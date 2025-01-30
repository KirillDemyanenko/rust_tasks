fn find_short(s: &str) -> u32 {
    let mut  words = s.split_whitespace().collect::<Vec<&str>>();
    words.sort_by(|a, b| a.len().cmp(&b.len()));
    words[0].len() as u32
}

fn main() {
    println!("{}", find_short("bitcoin take over the world maybe who knows perhaps"));
}
