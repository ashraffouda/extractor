use dotext::Docx;
use dotext::MsDoc;
use std::io::Read;

pub fn extract_text(path: &str) {
    let mut file = Docx::open(path).unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    println!("{}", content);
}
