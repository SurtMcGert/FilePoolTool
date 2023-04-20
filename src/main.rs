use native_dialog::{FileDialog, MessageDialog, MessageType};
use std::fs::{self, DirEntry};
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    //get user to select input directory
    println!("selecting input directory");
    let inputPath = FileDialog::new()
        .set_filename("select input directory")
        .set_location("~/Documents")
        .show_open_single_dir()
        .unwrap();

    let inputPath = match inputPath {
        Some(inputPath) => inputPath,
        None => return,
    };

    let yes = MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("Do you want to open the file?")
        .set_text(&format!("{:#?}", inputPath))
        .show_confirm()
        .unwrap();

    if yes {
        //get user to select output directory
        println!("selecting output directory");
        let outputPath = FileDialog::new()
            .set_filename("select output directory")
            .set_location("~/Documents")
            .show_open_single_dir()
            .unwrap();

        let outputPath = match outputPath {
            Some(outputPath) => outputPath,
            None => return,
        };

        let yes = MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title("Do you want to open the file?")
            .set_text(&format!("{:#?}", outputPath))
            .show_confirm()
            .unwrap();

        if yes {
            println!("copying the files");
            let result = copyFiles(inputPath.as_path(), outputPath.as_path());
            match result {
                Ok((numOfCopiedFiles)) => println!("number of files copied: {}", numOfCopiedFiles),
                Err(e) => panic!("Error: {}", e),
            }

            pause();
        }
    }
}

//function to copy all the files from one directory and all its sub-directories to another
fn copyFiles(input: &Path, output: &Path) -> io::Result<(i32)> {
    let mut currentNumOfCopiedFiles: i32 = 0;
    if (input.is_dir() && output.is_dir()) {
        // Iterate over the files in the current directory.
        for entry in fs::read_dir(input)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let result = copyFiles(&path, output);
                match result {
                    Ok((numOfCopiedFiles)) => currentNumOfCopiedFiles += numOfCopiedFiles,
                    Err(e) => panic!("Error: {}", e),
                }
            } else {
                println!("{}", path.to_string_lossy());
                let filename = path.file_name().unwrap();
                let filenamePath = Path::new(filename);
                let outputPath = output.join(filenamePath);
                let result = fs::copy(path, outputPath);
                currentNumOfCopiedFiles += 1;
            }
        }
    }
    Ok((currentNumOfCopiedFiles))
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
