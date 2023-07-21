use std::{fs, env, io, fs::Metadata, os::unix::prelude::PermissionsExt};

mod colors;
use colors::Colors;

mod args;
use args::CommandLineArgs;

#[derive(Debug)]
struct Dir {
    path: String,
    files: Vec<FileEntry>,
}

#[derive(Debug)]
struct FileEntry {
    file_path: String,
    meta: Metadata
}

impl FileEntry {
    fn new(path: &String) -> FileEntry {
        let meta = fs::metadata(&path); 

        FileEntry {
            file_path: path.to_string(),
            meta: meta.unwrap()
        }
    }

    fn display_permissions(&self) -> String {
        let permissions_decimal = self.meta.permissions().mode();
        let mut permissions_string = String::from(format!("{:b}", &permissions_decimal));
        permissions_string = permissions_string[permissions_string.len() - 9..].to_string();

        let mut i: i32 = 0;
        let mut output: String = String::new();
        for x in permissions_string.chars() {
            // match the current char
            i = i + 1;
            match i {
                1 => {
                    output.push(if x == '1' {'r'} else {'-'});
                }
                2 => {
                    output.push(if x == '1' {'w'} else {'-'});
                }
                _ => {
                    output.push(if x == '1' {'x'} else {'-'});
                    i = 0; 
                }
            };
        }
        return output;
    }
}

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    let parsed_args = CommandLineArgs::new(&args);

    let files = fs::read_dir(".")?.map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>, io::Error>>()?;
    let mut files_list: Vec<FileEntry> = Vec::new();

    for file in files{
        let f: FileEntry = FileEntry::new(&file.display().to_string());
        files_list.push(f); 
    }

    let cwd = Dir {
        path: env::current_dir().unwrap().display().to_string(),
        files: files_list     
    };

    printc!("Current Dir: ", green);
    println!("{}", cwd.path);

    printlnc!("Files in Dir:",  l_blue);
    for file in cwd.files {
        print!("{} ", file.display_permissions());

        let file_size = &file.meta.len();
        print!("{:>width$} ", file_size, width=6);
    
        if parsed_args.show_owner {
            let text = "Owner";
            print!("{}", format!("{:^width$}", text, width = text.len() + 2));
        }

        if parsed_args.show_last_modified {
            let text = "last_modified";
            print!("{}", format!("{:^width$}", text, width = text.len() + 2));
        }

        let is_dir = file.meta.is_dir();
        if is_dir {
            printlnc!(&file.file_path, purple);
        } else {
            println!("{}", &file.file_path);
        }

    }
    Ok(())
}

