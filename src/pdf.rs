pub fn extract_text(path: &str) {
    let bytes = std::fs::read(path).unwrap();
    let out = pdf_extract::extract_text_from_mem(&bytes).unwrap();
    println!("${out}")
}
