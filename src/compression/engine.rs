use crate::compression::{CompressionSettings, CompressionResult, OutputFormat};
use crate::file::ImageFile;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Instant;

pub struct CompressionEngine;

impl CompressionEngine {
    pub fn compress_files(
        files: Vec<ImageFile>,
        settings: CompressionSettings,
        progress_sender: mpsc::Sender<CompressionProgress>,
    ) -> Vec<CompressionResult> {
        let mut results = Vec::new();
        let total_files = files.len();

        // Ensure output directory exists
        if let Err(e) = std::fs::create_dir_all(&settings.output_directory) {
            let _ = progress_sender.send(CompressionProgress::Error(format!(
                "Failed to create output directory: {}", e
            )));
            return results;
        }

        for (index, file) in files.into_iter().enumerate() {
            let _ = progress_sender.send(CompressionProgress::Processing {
                current: index + 1,
                total: total_files,
                filename: file.name.clone(),
            });

            let result = Self::compress_single_file(&file, &settings);
            results.push(result);

            let progress = (index + 1) as f32 / total_files as f32;
            let _ = progress_sender.send(CompressionProgress::Progress(progress));
        }

        let _ = progress_sender.send(CompressionProgress::Complete);
        results
    }

    fn compress_single_file(file: &ImageFile, settings: &CompressionSettings) -> CompressionResult {
        let start_time = Instant::now();
        
        let output_path = Self::generate_output_path(file, settings);
        
        match Self::perform_compression(file, &output_path, settings) {
            Ok(compressed_size) => {
                let processing_time = start_time.elapsed();
                CompressionResult::new(
                    file.path.clone(),
                    output_path,
                    file.size,
                    compressed_size,
                    processing_time,
                )
            }
            Err(error) => CompressionResult::failed(file.path.clone(), error),
        }
    }

    fn perform_compression(
        file: &ImageFile,
        output_path: &Path,
        settings: &CompressionSettings,
    ) -> Result<u64, String> {
        match settings.output_format {
            OutputFormat::Png => Self::compress_to_png(file, output_path, settings),
            OutputFormat::WebP => Self::compress_to_webp(file, output_path, settings),
            OutputFormat::Jpeg => Self::compress_to_jpeg(file, output_path, settings),
        }
    }

    fn compress_to_png(
        file: &ImageFile,
        output_path: &Path,
        settings: &CompressionSettings,
    ) -> Result<u64, String> {
        // Load and convert image to PNG
        let img = image::open(&file.path).map_err(|e| format!("Failed to load image: {}", e))?;
        
        // Save as PNG first
        let temp_path = output_path.with_extension("temp.png");
        img.save(&temp_path).map_err(|e| format!("Failed to save PNG: {}", e))?;

        // Optimize with oxipng
        let mut options = oxipng::Options::default();
        options.optimize_alpha = true;
        options.strip = if settings.preserve_metadata {
            oxipng::StripChunks::None
        } else {
            oxipng::StripChunks::Safe
        };

        match oxipng::optimize(&oxipng::InFile::Path(temp_path.clone()), &oxipng::OutFile::Path(Some(output_path.to_path_buf())), &options) {
            Ok(_) => {
                let _ = std::fs::remove_file(temp_path);
                let metadata = std::fs::metadata(output_path).map_err(|e| format!("Failed to read output size: {}", e))?;
                Ok(metadata.len())
            }
            Err(e) => {
                let _ = std::fs::remove_file(temp_path);
                Err(format!("PNG optimization failed: {}", e))
            }
        }
    }

    fn compress_to_webp(
        file: &ImageFile,
        output_path: &Path,
        settings: &CompressionSettings,
    ) -> Result<u64, String> {
        let img = image::open(&file.path).map_err(|e| format!("Failed to load image: {}", e))?;
        
        let rgba_img = img.to_rgba8();
        let (width, height) = rgba_img.dimensions();

        let encoder = webp::Encoder::from_rgba(&rgba_img, width, height);
        let encoded = encoder.encode(settings.webp_quality);

        std::fs::write(output_path, &*encoded).map_err(|e| format!("Failed to write WebP: {}", e))?;
        
        Ok(encoded.len() as u64)
    }

    fn compress_to_jpeg(
        file: &ImageFile,
        output_path: &Path,
        settings: &CompressionSettings,
    ) -> Result<u64, String> {
        let img = image::open(&file.path).map_err(|e| format!("Failed to load image: {}", e))?;
        
        let rgb_img = img.to_rgb8();
        let mut output = std::fs::File::create(output_path).map_err(|e| format!("Failed to create output file: {}", e))?;
        
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, settings.jpeg_quality);
        let (width, height) = rgb_img.dimensions();
        
        encoder.encode(&rgb_img, width, height, image::ColorType::Rgb8)
            .map_err(|e| format!("JPEG encoding failed: {}", e))?;

        let metadata = std::fs::metadata(output_path).map_err(|e| format!("Failed to read output size: {}", e))?;
        Ok(metadata.len())
    }

    fn generate_output_path(file: &ImageFile, settings: &CompressionSettings) -> PathBuf {
        let extension = match settings.output_format {
            OutputFormat::Png => "png",
            OutputFormat::WebP => "webp",
            OutputFormat::Jpeg => "jpg",
        };

        let stem = file.path.file_stem().unwrap_or_default();
        let filename = format!("{}_compressed.{}", stem.to_string_lossy(), extension);
        
        Path::new(&settings.output_directory).join(filename)
    }
}

#[derive(Clone, Debug)]
pub enum CompressionProgress {
    Processing { current: usize, total: usize, filename: String },
    Progress(f32),
    Complete,
    Error(String),
}
