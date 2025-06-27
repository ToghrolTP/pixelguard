#[derive(Clone, Debug, PartialEq)]
pub enum OutputFormat {
    Png,
    WebP,
    Jpeg,
}

#[derive(Clone, Debug)]
pub struct CompressionSettings {
    pub output_format: OutputFormat,
    pub png_level: u8,     // 1-6
    pub webp_quality: f32, // 0.0-100.0
    pub jpeg_quality: u8,  // 1-100
    pub preserve_metadata: bool,
    pub output_directory: String,
}

impl Default for CompressionSettings {
    fn default() -> Self {
        Self {
            output_format: OutputFormat::Png,
            png_level: 6,
            webp_quality: 80.0,
            jpeg_quality: 85,
            preserve_metadata: false,
            output_directory: "output".to_string(),
        }
    }
}

impl CompressionSettings {
    pub fn validate(&self) -> Result<(), String> {
        if self.png_level < 1 || self.png_level > 6 {
            return Err("PNG level must be 1-6".to_string());
        }
        if self.webp_quality < 0.0 || self.webp_quality > 100.0 {
            return Err("WebP quality must be 0-100".to_string());
        }
        if self.jpeg_quality < 1 || self.jpeg_quality > 100 {
            return Err("JPEG quality must be 1-100".to_string());
        }
        Ok(())
    }
}
