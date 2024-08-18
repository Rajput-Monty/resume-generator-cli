
# Resume Generator CLI Tool

## Introduction

Welcome to the Resume Generator CLI Tool, a simple yet powerful command-line application designed to streamline the process of creating professional resumes. Built using Rust, this tool allows users to input their personal information, education, experience, and other relevant details to generate a well-formatted PDF resume. With features like file persistence, task management, and PDF generation, our tool offers a comprehensive solution for job seekers who want to create high-quality resumes with minimal effort.

## How the Tool Works

The Resume Generator CLI Tool guides users through a series of prompts where they can enter their details, such as personal information, education history, work experience, etc. Once all necessary data is collected, the tool compiles this information into a visually appealing and structured PDF document. Users can preview the resume, make any necessary edits, and then save the final version as a PDF. The tool also supports saving and loading sessions, allowing users to build their resume incrementally.

## Why Users Should Adopt This Tool

Our Resume Generator CLI Tool stands out for its simplicity, flexibility, and the quality of the final output. Unlike other resume generators that might limit formatting options or require a subscription, our tool is entirely open-source and customizable. Users have full control over their resume content, and the PDF output ensures compatibility across all platforms. Additionally, the tool's file persistence feature allows users to manage and update their resume with ease, making it a practical choice for both active job seekers and those looking to keep their resume up to date.

## Video Demo

<a href="https://www.youtube.com/watch?v=YOUR_VIDEO_ID_HERE">
  <img src="https://raw.githubusercontent.com/Rajput-Monty/resume-generator-cli/master/Demoimg" alt="Watch the Demo" width="450" height="300">
</a>

Click the image above to watch a short demo of the Resume Generator CLI Tool in action. In this video, we demonstrate how to input information, preview the resume, and generate the final PDF document.

## Installation Instructions

### Prerequisites

- Rust installed on your system. If you don't have Rust installed, you can get it from [rust-lang.org](https://www.rust-lang.org/).
- `cargo` package manager for Rust.

### Building the Application

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/Rajput-Monty/resume-generator-cli.git
   cd resume-generator-cli
   ```

2. **Build the Application:**

   ```bash
   cargo build --release
   ```

3. **Run the Application:**

   ```bash
   cargo run
   ```

### Usage

1. **Start the Resume Generator:**

   Run the tool and follow the on-screen prompts to enter your personal information, education, experience, and other resume sections.

2. **Generate PDF:**

   After completing the inputs, choose the option to generate the PDF resume. The file will be saved in the current directory with the name `resume.pdf`.

3. **Preview and Edit:**

   You can preview the generated resume by running

   ```bash
   cargo run -- --view
   ```

### Source Files

All source files are included in this repository. You can explore the codebase, customize the application, or even contribute to the project. The repository is structured as follows:

```
resume-generator-cli/
├── src/
│   ├── main.rs
├── Cargo.toml
├── resume.pdf
└── README.md
```

Feel free to reach out via Issues or Pull Requests if you have any suggestions or improvements!
