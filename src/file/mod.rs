use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct ImageFile {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub format: ImageFormat,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ImageFormat {
    Jpeg,
    Png,
    WebP,
    Gif,
    Bmp,
    Tiff,
    Unknown,
}

pub struct FileManager;

impl FileManager {
    pub fn open_file_dialog() -> Option<Vec<PathBuf>> {
        rfd::FileDialog::new()
            .add_filter("Images", &["jpg", "jpeg", "png", "webp", "gif", "bmp", "tiff"])
            .set_title("Select Images")
            .pick_files()
    }

    pub fn analyze_file(path: PathBuf) -> Option<ImageFile> {
        let metadata = std::fs::metadata(&path).ok()?;
        let name = path.file_name()?.to_string_lossy().to_string();
        let format = Self::detect_format(&path);
        
        Some(ImageFile {
            path,
            name,
            size: metadata.len(),
            format,
        })
    }

    fn detect_format(path: &PathBuf) -> ImageFormat {
        match path.extension().and_then(|s| s.to_str()) {
            Some("jpg") | Some("jpeg") => ImageFormat::Jpeg,
            Some("png") => ImageFormat::Png,
            Some("webp") => ImageFormat::WebP,
            Some("gif") => ImageFormat::Gif,
            Some("bmp") => ImageFormat::Bmp,
            Some("tiff") | Some("tif") => ImageFormat::Tiff,
            _ => ImageFormat::Unknown,
        }
    }

    pub fn format_file_size(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        format!("{:.1} {}", size, UNITS[unit_index])
    }
}
