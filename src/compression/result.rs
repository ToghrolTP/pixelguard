use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct CompressionResult {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub original_size: u64,
    pub compressed_size: u64,
    pub compression_ratio: f32,
    pub status: CompressionStatus,
    pub processing_time: std::time::Duration,
}

#[derive(Clone, Debug)]
pub enum CompressionStatus {
    Success,
    Failed(String),
    Skipped(String),
}

impl CompressionResult {
    pub fn new(
        input_path: PathBuf,
        output_path: PathBuf,
        original_size: u64,
        compressed_size: u64,
        processing_time: std::time::Duration,
    ) -> Self {
        let compression_ratio = if original_size > 0 {
            1.0 - (compressed_size as f32 / original_size as f32)
        } else {
            0.0
        };

        Self {
            input_path,
            output_path,
            original_size,
            compressed_size,
            compression_ratio,
            status: CompressionStatus::Success,
            processing_time,
        }
    }

    pub fn failed(input_path: PathBuf, error: String) -> Self {
        Self {
            input_path,
            output_path: PathBuf::new(),
            original_size: 0,
            compressed_size: 0,
            compression_ratio: 0.0,
            status: CompressionStatus::Failed(error),
            processing_time: std::time::Duration::from_secs(0),
        }
    }

    pub fn space_saved_bytes(&self) -> u64 {
        if self.original_size > self.compressed_size {
            self.original_size - self.compressed_size
        } else {
            0
        }
    }
}
