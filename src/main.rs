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

#[derive(Debug, Clone)]
struct FileEntry {
    file_path: String,
    meta: Metadata,
    file_size: u64
}

impl FileEntry {
    fn new(path: &str) -> FileEntry {
        let meta = fs::metadata(&path).unwrap(); 

        FileEntry {
            file_path: path.to_string(),
            meta: meta.clone(),
            file_size: meta.len()  
        }
    }

    fn display_permissions(&self) -> String {
        let mut permissions_string = String::from(format!("{:b}", self.meta.permissions().mode()));
        permissions_string = permissions_string[permissions_string.len() - 9..].to_string();

        let mut output: String = String::from("rwxrwxrwx");
        for (i,x) in permissions_string.chars().enumerate() {
            if x == '0'{
                output
                    .replace_range(output
                                   .char_indices()
                                   .nth(i)
                                   .map(|(pos, ch)| (pos..pos + ch.len_utf8()))
                                   .unwrap(), "-");
            }
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
        files: files_list.clone()     
    };

    printc!("Current Dir: ", green);
    println!("{}", cwd.path);

    printlnc!("Files in Dir:",  l_blue);
    for file in cwd.files {
        print!("{} ", file.display_permissions());

        let column_width = files_list.iter().max_by_key(|s| s.file_size);
        print!("{:<width$} ", file.file_size, width=column_width.unwrap().file_size.to_string().len());
    
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

