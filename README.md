<p align="center"><img src="https://i.imgur.com/Qv49B0b.png"></p>

# Jotspot - Text Annotation Tool

Jotspot is a simple text annotation tool that allows you to create and manage text annotations with timestamps. Whether you want to jot down quick notes, timestamps for important events, or simply annotate text for future reference, Jotspot has got you covered.

## Features

- **Annotation Creation:** Easily create text annotations along with timestamps.
- **Timestamps:** Annotations include timestamps indicating when they were created.
- **User-Friendly Interface:** An intuitive and user-friendly text-based interface for annotation management.
- **List Annotations:** View and manage your annotations conveniently.
- **Customizable:** Customize your annotation text as you like it.
- **Timestamp Formatting:** Jotspot provides user-friendly timestamp formatting like "X seconds ago," making it easy to understand when an annotation was created.

## Getting Started

### Prerequisites

Before you get started, ensure you have the following installed:

- Rust (if you don't have it, you can [get it here](https://www.rust-lang.org/tools/install))
- Git (for cloning the repository)

### Installation

1. Clone the "jotspot" repository to your local machine:

   ```bash
   git clone https://github.com/ricardofares/jotspot.git
   ```

2. Change to the "jotspot" project directory:

   ```bash
   cd jotspot
   ```
   
3. Build the project:

   ```bash
   cargo build --release
   ```

  As a result, the project binary can be found in the target/release/ directory.

## Usage

Use the jotspot command followed by your text to create a new annotation. For example:
  
  ```bash
  ./jotspot "This is my first annotation."
  ```

Run jotspot without arguments to enter the interactive annotation interface:

  ```bash
  ./jotspot
  ```

Use the interactive interface to view and manage your annotations.

## Contributing

Contributions are welcome! If you want to contribute to Jotspot, please follow these steps:

 1. Fork the repository.
 2. Create a new branch for your feature or bug fix.
 3. Make your changes and commit them with clear, concise messages.
 4. Push your changes to your fork.
 5. Submit a pull request to the main repository.
    
## Acknowledgments

- The [Cursive](https://github.com/gyscos/cursive) TUI Library for providing the user interface components.
- The Rust community for creating a powerful and efficient programming language.
