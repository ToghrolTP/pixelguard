# PixelGuard 🖼️

PixelGuard is a user-friendly, high-performance desktop application for image compression. Built with Rust and the `egui` framework, it provides a simple interface to help you reduce the file size of your images without sacrificing quality.

## ✨ Features

  * **Multiple Formats**: Compress images to **PNG**, **WebP**, and **JPEG**.
  * **Adjustable Quality**: Fine-tune the compression with quality sliders for JPEG/WebP and optimization levels for PNG.
  * **Responsive UI**: The compression engine runs on a separate thread, ensuring the user interface never freezes, even during intensive processing.
  * **Detailed Results**: Get a clear summary of the compression results, including original size, compressed size, and the percentage of space saved for each file.
  * **File Management**: Easily add multiple files, view them in a list, and clear the queue when needed.

-----

## 🚀 Getting Started

Follow these instructions to build and run PixelGuard from the source code.

### Prerequisites

  * **Rust**: Ensure you have the Rust programming language and its package manager, Cargo, installed. You can get them from [rust-lang.org](https://www.rust-lang.org/).

### Installation & Running

1.  **Clone the repository:**

    ```sh
    git clone https://github.com/toghroltp/pixelguard.git
    cd pixelguard
    ```

2.  **Build and run the application:**

    ```sh
    cargo run --release
    ```

    The `--release` flag compiles the application with optimizations, providing the best performance.

-----

## 📋 How to Use

1.  **Select Images**: Click the **"Browse Files"** button to open a file dialog and select the images you want to compress.
2.  **Configure Settings**:
      * Choose an output **Format** (PNG, WebP, or JPEG).
      * Adjust the **Quality** or **Level** slider to your preference.
      * Set the **Output** directory where the compressed files will be saved.
3.  **Compress**: Click the **"Compress Images"** button to start the process.
4.  **View Results**: The results panel will show a summary and a detailed breakdown of the compression savings. You can also click **"Open folder"** to view the files directly.

-----

## 🛠️ Technologies Used

PixelGuard is built with a modern set of Rust libraries:

  * **`eframe` / `egui`**: For the immediate mode graphical user interface.
  * **`image`**: For robust image loading and encoding.
  * **`oxipng`**: For advanced, lossless PNG optimization.
  * **`webp`**: For encoding images into the modern WebP format.
  * **`rfd`**: For cross-platform, native file dialogs.

-----

## 📂 Project Structure

The codebase is organized into logical modules for clarity and maintainability:

```
src
├── app.rs              # Main application struct and eframe loop
├── main.rs             # Application entry point
├── compression/        # Core compression logic and settings
│   ├── engine.rs
│   ├── result.rs
│   └── settings.rs
├── file/               # File analysis and management
│   └── mod.rs
└── ui/                 # All GUI components and state
    ├── state.rs
    └── components/
        ├── compression_panel.rs
        ├── file_input.rs
        ├── header.rs
        └── output_panel.rs
```

-----

## 🤝 Contributing

Contributions are welcome\! If you have ideas for new features or have found a bug, feel free to open an issue or submit a pull request.

## 📜 License

This project is licensed under the MIT License. See the `LICENSE` file for details.
