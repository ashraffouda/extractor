pub fn extract_text(path: &str) {
    let bytes = std::fs::read(path).unwrap();
    let out = html2text::from_read(&bytes[..], 80);
    println!("{}", out);
}
