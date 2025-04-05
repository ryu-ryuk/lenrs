use tesseract::Tesseract;

// use tesseract 
//
pub fn extract_text(image_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut tesseract = Tesseract::new(None, Some("eng"))?
        .set_image(image_path)?; // returns Tesseract

    let text = tesseract.get_text()?;
    Ok(text)
}
