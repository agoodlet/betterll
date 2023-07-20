use std::{fs, env, io, fs::Metadata, os::unix::prelude::PermissionsExt};

mod colors;
use colors::Colors;

#[derive(Debug)]
struct Dir {
    path: String,
    files: Vec<File>,
}

// TODO rename this struct to something like FileEntry to be more clear that this isn't an object
// that is a file on the disk, but just the entry of a file for us to display
#[derive(Debug)]
struct File {
    file_path: String,
    meta: Metadata
}

impl File {
    fn new(path: &String) -> File{
        let meta = fs::metadata(&path); 

        File {
            file_path: path.to_string(),
            meta: meta.unwrap()
        }
    }
}

fn main() -> io::Result<()> {

    let files = fs::read_dir(".")?.map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>, io::Error>>()?;
    let mut files_list: Vec<File> = Vec::new();

    for file in files{
        let f: File = File::new(&file.display().to_string());
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
        let permissions_decimal = &file.meta.permissions().mode();
        let permissions_string = String::from(format!("{:b}", &permissions_decimal));
        print!("{} ", translate_binary_permission(permissions_string[permissions_string.len() - 9..].to_string()));

        let is_dir = file.meta.is_dir();
        if is_dir {
            printlnc!(&file.file_path, purple);
        } else {
            println!("{}", &file.file_path);
        }

    }
    

    Ok(())
}

fn translate_binary_permission(permission: String)  -> String {
    let mut i: i32 = 0;
    let mut output: String = String::new();
    for x in permission.chars() {
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
