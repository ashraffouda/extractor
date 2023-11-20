mod docx;
mod html;
mod pdf;
use core::panic;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("please provide path to the file you want to extract text from")
    }
    let file_path = std::path::Path::new(&args[1]);
    match file_path.extension() {
        None => panic!("Please provide file extension in file name"),
        Some(ext) => match ext.to_str() {
            Some("pdf") => pdf::extract_text(&args[1]),
            Some("docx") => docx::extract_text(&args[1]),
            Some("html") => html::extract_text(&args[1]),
            _ => panic!("unsupported file extension"),
        },
    }
}
