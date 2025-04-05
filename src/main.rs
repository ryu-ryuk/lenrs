mod core;

use core::{ capture, clipboard, ocr };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let coords = capture::get_selection_area();
    let (path, _tmpfile) = capture::screenshot_area(&coords);
    let text = ocr::extract_text(&path)?;

    println!("Extracted text:\n{}", text);
    clipboard::copy_to_clipboard(&text);

    Ok(())
}

