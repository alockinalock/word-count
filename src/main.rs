use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

pub mod path;
pub mod pdf;

use path::*;
use pdf::parse::*;
use pdf::read::*;
use pdf::write::*;

use std::io;

extern crate regex;

lazy_static::lazy_static! {
    pub static ref ROOT_DIR: PathBuf = root_dir().expect("Failed to get root folder");

    pub static ref PDF_DIR: PathBuf = ROOT_DIR.join("assets");

    pub static ref SAVE_DIR: PathBuf = ROOT_DIR.join("saved");
}

struct Config {
    sorted: bool,
    saved: bool,
}

fn main() {
    let mut userConfig = Config {
        sorted: false,
        saved: false,
    };

    // Created saved folder if it doesn't exist
    if !SAVE_DIR.exists() {
        std::fs::create_dir(SAVE_DIR.as_path()).expect("Failed to create saved folder");
        println!("The saved folder was not found. Folder created at {}", SAVE_DIR.to_string_lossy());
    }

    // this line may panic
    let pdfs = get_pdfs(&PDF_DIR).unwrap();

    gui_list_pdfs_ordered(&pdfs);

    println!("Enter the number which represents the PDF document you would like to use.");

    let mut temp_input_holder: String = String::new();
    io::stdin().read_line(&mut temp_input_holder).expect("Failed to read input");

    let temp_int_holder: usize = temp_input_holder.trim().parse().unwrap();

    let selected_pdf: PathBuf = PDF_DIR.join(pdfs.get(temp_int_holder).unwrap().clone());

    temp_input_holder.clear();
    println!("Would you like for the list to be sorted? [This will apply to both printing and saving]");
    print!("[yes/no] ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut temp_input_holder).expect("Failed to read input");

    if temp_input_holder.trim().to_lowercase().starts_with("y") {
        userConfig.sorted = true;
    }
    else {
        userConfig.sorted = false;
    }

    temp_input_holder.clear();
    println!("Would you like for the list to be saved? [This will save as JSON]");
    print!("[yes/no] ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut temp_input_holder).expect("Failed to read input");

    if temp_input_holder.trim().to_lowercase().starts_with("y") {
        userConfig.saved = true;
    }
    else {
        userConfig.saved = false;
    }

    println!("*** Reading selected pdf\n    CONFIG\n    --sorted: {}\n    --saved: {}", userConfig.sorted, userConfig.saved);

    let data: String = read_pdf(&selected_pdf);

    let word_counts: HashMap<String, usize> = regex_parse_for_words(&data);

    // there's probably some better alternative... oh well!
    if userConfig.sorted {
        let mut sorted_count: Vec<(String, usize)> = sort_by_instances(word_counts);

        temp_input_holder.clear();
        println!("Some terminals limit the amount of characters able to be printed. Would you like for the order to be reversed so you can prioritize the words with the most instances being shown?");
        print!("[yes/no] ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut temp_input_holder).expect("Failed to read input");

        if temp_input_holder.trim().to_lowercase().starts_with("y") {
            sorted_count.reverse();
        }

        for (word, count) in sorted_count {
            println!("{}: {}", word, count);
        }
    }
    else {
        for (word, count) in word_counts {
            println!("{}: {}", word, count);
           }
    }

    if userConfig.saved {
        println!("JSON saved at: {}", SAVE_DIR.to_string_lossy());
    }

    // let sorted_word_counts = sort_by_instances(word_counts);

    // let reversed_swc = sorted_word_counts.iter().rev();

    // for (word, count) in reversed_swc {
    //     println!("{}: {}", word, count);
    // }
}

fn gui_list_pdfs_ordered(vec: &Vec<String>) {
    let mut iterator_count = 0;
    for item in vec {
        println!("{}) {}", iterator_count, item);
        iterator_count += 1;
    }
    
    if iterator_count <= 0 {
        println!("Please check if there are any PDFs in the directory: {}", PDF_DIR.to_string_lossy())
    }
}