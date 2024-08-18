use clap::{Command, Arg};
use printpdf::{PdfDocument, PdfLayerReference, IndirectFontRef, BuiltinFont, Mm};
use regex::Regex;
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufWriter};
use std::process::Command as ProcessCommand;

// Data Models

/// Represents personal information of an individual
#[derive(Serialize, Deserialize, Debug)]
struct PersonalInfo {
    name: String,
    sex: String,
    age: u32,
    religion: String,
    birthdate: String,
    fathers_name: String,
    mothers_name: String,
    marital_status: String,
    phone_number: String,
    email: String,
    address: String,
}

/// Represents an educational qualification
#[derive(Serialize, Deserialize, Debug)]
struct Education {
    degree: String,
    institution: String,
    year: String,
}

/// Represents a work experience entry
#[derive(Serialize, Deserialize, Debug)]
struct Experience {
    job_title: String,
    company: String,
    duration: String,
    responsibilities: String,
    expected_salary: String,
}

/// Represents a complete resume
#[derive(Serialize, Deserialize, Debug)]
struct Resume {
    personal_info: PersonalInfo,
    education: Vec<Education>,
    experience: Vec<Experience>,
}


// Function to prompt user for input

/// Prompts the user for input with optional regex validation
fn prompt_for_input(prompt: &str, regex: Option<&Regex>, error_msg: &str) -> String {
    let mut input = String::new();
    loop {
        println!("{}", prompt);
        io::stdin().read_line(&mut input).unwrap();
        let trimmed_input = input.trim().to_string();
        
        match regex {
            Some(re) if !re.is_match(&trimmed_input) => {
                println!("{}", error_msg);
                input.clear();
                continue;
            },
            _ => break,
        }
    }
    input.trim().to_string()
}

// Function to create resume from user input

/// Creates a resume by prompting the user for various details
fn create_resume() -> Resume {
    let name_regex = Regex::new(r"^[a-zA-Z\s]+$").unwrap();
    let age_regex = Regex::new(r"^\d+$").unwrap();
    let phone_regex = Regex::new(r"^\+?\d+(-\d+)*$").unwrap(); 
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap(); 

    let name = prompt_for_input(
        "Enter your name:",
        Some(&name_regex),
        "Invalid name. Only letters and spaces are allowed."
    );
    let sex = prompt_for_input(
        "Enter your sex:",
        Some(&name_regex),
        "Invalid sex. Only letters and spaces are allowed."
    );
    let age = prompt_for_input(
        "Enter your age:",
        Some(&age_regex),
        "Invalid age. Please enter a valid number."
    );
    let religion = prompt_for_input(
        "Enter your religion:",
        Some(&name_regex),
        "Invalid religion. Only letters and spaces are allowed."
    );
    let birthdate = prompt_for_input("Enter your birthdate:",
        None, 
        "Invalid date format."
    );
    let fathers_name = prompt_for_input(
        "Enter your father's name:",
        Some(&name_regex),
        "Invalid name. Only letters and spaces are allowed."
    );
    let mothers_name = prompt_for_input(
        "Enter your mother's name:",
        Some(&name_regex),
        "Invalid name. Only letters and spaces are allowed."
    );
    let marital_status = prompt_for_input(
        "Enter your marital status:",
        Some(&name_regex),
        "Invalid status. Only letters and spaces are allowed."
    );
    let phone_number = prompt_for_input(
        "Enter your phone number:",
        Some(&phone_regex),
        "Invalid phone number format."
    );
    let email = prompt_for_input(
        "Enter your email:",
        Some(&email_regex),
        "Invalid email format."
    );
    let address = prompt_for_input(
        "Enter your address:",
        None,
        "Invalid address format."
    );

    let personal_info = PersonalInfo {
        name,
        sex,
        age: age.parse().unwrap_or(0),
        religion,
        birthdate,
        fathers_name,
        mothers_name,
        marital_status,
        phone_number,
        email,
        address,
    };

    let degree = prompt_for_input(
        "Enter your degree:",
        Some(&name_regex),
        "Invalid degree. Only letters and spaces are allowed."
    );
    let institution = prompt_for_input(
        "Enter your institution:",
        Some(&name_regex),
        "Invalid institution. Only letters and spaces are allowed."
    );
    let year = prompt_for_input(
        "Enter the year of graduation:",
        Some(&age_regex),
        "Invalid year. Please enter a valid year."
    );

    let education = vec![Education {
        degree,
        institution,
        year,
    }];
    
    let job_title = prompt_for_input(
        "Enter your job title:",
        Some(&name_regex),
        "Invalid job title. Only letters and spaces are allowed."
    );
    let company = prompt_for_input(
        "Enter your company:",
        Some(&name_regex),
        "Invalid company name. Only letters and spaces are allowed."
    );
    let duration = prompt_for_input(
        "Enter your job duration:",
        None, 
         "Invalid duration format."
        );
    let responsibilities = prompt_for_input(
        "Enter your responsibilities:",
        None,
        "Invalid responsibilities format."
    );
    let expected_salary = prompt_for_input(
        "Enter your expected salary:",
        None,
        "Invalid salary format."
    );

    let experience = vec![Experience {
        job_title,
        company,
        duration,
        responsibilities,
        expected_salary,
    }];

    Resume {
        personal_info,
        education,
        experience,
    }
}

// Function to save resume as a PDF

/// Saves the resume data to a PDF file
fn save_resume_to_pdf(resume: &Resume) -> Result<(), printpdf::Error> {
    // Create a new PDF document
    let (doc, page1, layer1) = PdfDocument::new("Resume", Mm(210.0), Mm(297.0), "Layer 1");

    // Load fonts
    let font_bold = doc.add_builtin_font(BuiltinFont::TimesBold)?;
    let font_light = doc.add_builtin_font(BuiltinFont::TimesRoman)?;
    let font_italic = doc.add_builtin_font(BuiltinFont::TimesItalic)?;

    // Get a reference to the layer
    let page = doc.get_page(page1);
    let layer = page.get_layer(layer1);

    // Helper function to add text with styling
    fn add_styled_text(
        layer: &PdfLayerReference,
        text: &str,
        font: &IndirectFontRef,
        size: f32,
        x: f32,
        y: f32
    ) {
        layer.use_text(
            text,
            size,
            Mm(x),
            Mm(y),
            font
        );
    }

    // heading to the PDF with bold font
    add_styled_text(
        &layer,
        "Professional Resume",
        &font_bold,
        24.0, // Font size for heading
        55.0, // Center of the page (width: 210 mm, so 105 mm from left)
        280.0 // Near the top of the page
    );

    // A hardcoded line just below the header
    add_styled_text(
        &layer,
        "_____________________________________________",
        &font_bold,
        24.0, // Font size for line
        10.0, // Line start x position
        270.0 // Line y position, just below the header
    );

    // Personal Information section with heading
    add_styled_text(
        &layer,
        "Personal Information",
        &font_bold,
        18.0, 
        10.0,
        250.0
    );

    let y_start = 235.0;
    let mut y_position = y_start;

    add_styled_text(
        &layer,
        &format!("Name: {}", resume.personal_info.name),
        &font_light,
        12.0,
        10.0,
        y_position
    );
    y_position -= 10.0;

    add_styled_text(
        &layer,
        &format!("Sex: {}", resume.personal_info.sex),
        &font_light,
        12.0,
        10.0,
        y_position
    );
    y_position -= 10.0;

    add_styled_text(
        &layer,
        &format!("Age: {}", resume.personal_info.age),
        &font_light,
        12.0,
        10.0,
        y_position
    );
    y_position -= 10.0;

    add_styled_text(
        &layer,
        &format!("Religion: {}", resume.personal_info.religion),
        &font_light,
        12.0,
        10.0,
        y_position
    );
    y_position -= 10.0;

    add_styled_text(
        &layer,
        &format!("Birthdate: {}", resume.personal_info.birthdate),
        &font_light,
        12.0,
        10.0,
        y_position
    );
    y_position -= 10.0;

    add_styled_text(
        &layer,
        &format!("Father's Name: {}", resume.personal_info.fathers_name),
        &font_light,
        12.0,
        10.0,
        y_position
    );
    y_position -= 10.0;

    add_styled_text(
        &layer,
        &format!("Mother's Name: {}", resume.personal_info.mothers_name),
        &font_light,
        12.0,
        10.0,
        y_position
    );
    y_position -= 10.0;

    add_styled_text(
        &layer,
        &format!("Marital Status: {}", resume.personal_info.marital_status),
        &font_light,
        12.0,
        10.0,
        y_position
    );
    y_position -= 10.0; 

    add_styled_text(
        &layer,
        &format!("Phone Number: {}", resume.personal_info.phone_number),
        &font_light,
        12.0,
        10.0,
        y_position
    );
    y_position -= 10.0;

    add_styled_text(
        &layer,
        &format!("Email: {}", resume.personal_info.email),
        &font_light,
        12.0,
        10.0,
        y_position
    );
    y_position -= 10.0;

    add_styled_text(
        &layer,
        &format!("Address: {}", resume.personal_info.address),
        &font_light,
        12.0,
        10.0,
        y_position
    );
    y_position -= 10.0;


    // Education section with heading
    add_styled_text(
        &layer,
        "Education",
        &font_bold,
        14.0,
        10.0,
        y_position
    );

    y_position -= 12.0;
    let mut x_position = 10.0;
    for e in &resume.education {
        add_styled_text(
            &layer,
            &format!("Degree: {}", e.degree),
            &font_light,
            12.0,
            x_position,
            y_position
        );
        x_position += 60.0; 

        add_styled_text(
            &layer,
            &format!("Institution: {}", e.institution),
            &font_light,
            12.0,
            x_position,
            y_position
        );
        x_position += 60.0; 

        add_styled_text(
            &layer,
            &format!("Year: {}", e.year),
            &font_light,
            12.0,
            x_position,
            y_position
        );
        x_position = 10.0; 

        y_position -= 15.0; 
    }

    //  Experience section with heading and line separator
    add_styled_text(
        &layer,
        "Experience",
        &font_bold,
        14.0,
        10.0,
        y_position
    );

    y_position -= 15.0;
    for e in &resume.experience {
        add_styled_text(
            &layer,
            &format!("Job Title: {}", e.job_title),
            &font_light,
            12.0,
            10.0,
            y_position
        );
        y_position -= 10.0;

        add_styled_text(
            &layer,
            &format!("Company: {}", e.company),
            &font_light,
            12.0,
            10.0,
            y_position
        );
        y_position -= 10.0;

        add_styled_text(
            &layer,
            &format!("Duration: {}", e.duration),
            &font_light,
            12.0,
            10.0,
            y_position
        );
        y_position -= 10.0;

        add_styled_text(
            &layer,
            &format!("Responsibilities: {}", e.responsibilities),
            &font_light,
            12.0,
            10.0,
            y_position
        );
        y_position -= 10.0; 

        add_styled_text(
            &layer,
            &format!("Expected Salary: {}", e.expected_salary),
            &font_light,
            12.0,
            10.0,
            y_position
        );
        y_position -= 15.0;
    }

    // signature field
    add_styled_text(
        &layer,
        "Signature: _________________________",
        &font_light,
        12.0,
        10.0,
        y_position
    );
    y_position -= 10.0;

    // Date field
    add_styled_text(
        &layer,
        "Date: _____________________________",
        &font_light,
        12.0,
        10.0,
        y_position
    );
    y_position -= 10.0;

    // verification statement
    add_styled_text(
        &layer,
        "I hereby declare that all the information provided above is true and accurate to the best of my knowledge.",
        &font_italic,
        10.0,
        10.0,
        y_position
    );

    // Save the PDF file
    let file = File::create("resume.pdf")?;
    let mut buf_writer = BufWriter::new(file);
    doc.save(&mut buf_writer)?;

    Ok(())
}

// Function to view the resume (implementation needed)
fn view_resume(file_path: &str) -> Result<(), Box<dyn Error>> {
    // Try to open the file using the appropriate system command
    if cfg!(target_os = "windows") {
        ProcessCommand::new("cmd")
            .args(&["/C", "start", file_path])
            .spawn()?
            .wait()?;
    } else if cfg!(target_os = "macos") {
        ProcessCommand::new("open")
            .arg(file_path)
            .spawn()?
            .wait()?;
    } else if cfg!(target_os = "linux") {
        ProcessCommand::new("xdg-open")
            .arg(file_path)
            .spawn()?
            .wait()?;
    }

    Ok(())
}

// Main function
fn main() {
    let matches = Command::new("Resume Generator")
        .version("1.0")
        .author("monty") // You can change your name here ok.
        .about("Generates and previews resumes")
        .arg(Arg::new("view")
            .long("view")
            .help("View the generated resume"))
        .get_matches();

    let pdf_path = "resume.pdf";

    if matches.contains_id("view") {
        if let Err(e) = view_resume(pdf_path) {
            eprintln!("Error viewing resume: {}", e);
        }
    } else {
        let resume = create_resume();
        if let Err(e) = save_resume_to_pdf(&resume) {
            eprintln!("Error saving resume: {}", e);
        } else {
            println!("Resume saved successfully as resume.pdf.");
        }
    }
}