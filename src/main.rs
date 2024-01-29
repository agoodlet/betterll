use std::{fs, env, io, fs::Metadata, os::unix::prelude::PermissionsExt, path::PathBuf, fmt::Write};

mod colors;
use colors::Colors;

mod args;
use args::CommandLineArgs;

// TODO
// - Add symlink following
// - change color for executable files
// - seperate draw functionality from processing
//      - maybe create a struct that holds _all_ the data
//      and then implement a function for it that draws it all to terminal

#[cfg(unix)]
mod permissions {
    pub const S_IRUSR: u32 = 0o400;
    pub const S_IWUSR: u32 = 0o200;
    pub const S_IXUSR: u32 = 0o100;

    pub const S_IRGRP: u32 = 0o040;
    pub const S_IWGRP: u32 = 0o020;
    pub const S_IXGRP: u32 = 0o010;
    
    pub const S_IROTH: u32 = 0o004;
    pub const S_IWOTH: u32 = 0o002;
    pub const S_IXOTH: u32 = 0o001;
}
use permissions::*;

#[derive(Debug)]
struct Dir {
    path: String,
    files: Vec<FileEntry>,
}

#[derive(Debug, Clone)]
struct FileEntry {
    file_path: String,
    meta: Metadata,
    file_size: u64,
    is_dir: bool,
    owner: String,
    last_modified: String,
}

impl FileEntry {
    fn new(file: PathBuf) -> Self {
        let meta = fs::metadata(file.display().to_string()).unwrap(); 
        let is_dir = meta.is_dir();

        FileEntry {
            file_path: file.file_name().unwrap().to_ascii_lowercase().into_string().unwrap(),
            meta: meta.clone(),
            file_size: meta.len(),
            is_dir,
            owner: "test".to_string(),
            last_modified: "test".to_string(),
        }
    }

    fn get_permissions(&self) -> String {
        let perms = self.meta.permissions().mode();
        let mut result: String = String::new();

        result.push(if perms & S_IRUSR != 0 {'r'} else {'-'});
        result.push(if perms & S_IWUSR != 0 {'w'} else {'-'});
        result.push(if perms & S_IXUSR != 0 {'x'} else {'-'});
        result.push(if perms & S_IRGRP != 0 {'r'} else {'-'});
        result.push(if perms & S_IWGRP != 0 {'w'} else {'-'});
        result.push(if perms & S_IXGRP != 0 {'x'} else {'-'});

        result.push(if perms & S_IROTH != 0 {'r'} else {'-'});
        result.push(if perms & S_IWOTH != 0 {'w'} else {'-'});
        result.push(if perms & S_IXOTH != 0 {'x'} else {'-'});

        result
    }
}

struct Output {
     dir: Dir,
     show_owner: bool,
     show_last_modified: bool,
     column_width: usize,
}

// impl Output {
//     fn draw(&self) {
//         printlnc!("idk lol", green);
//
//         printlnc!(self.dir.path, green);
//     }
// }

fn main() -> io::Result<()> {
    let mut file_path = ".".to_string();
    let mut args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1].chars().nth(0).unwrap() != '-' {
            file_path = args[1].clone();
            args.remove(1);
        }
    }
    let parsed_args = CommandLineArgs::new(&args);

    let mut output = Output {
        dir: Dir {
            path: file_path,
            files: Vec::new()
        },
        show_owner: parsed_args.show_owner,
        show_last_modified: parsed_args.show_last_modified,
        column_width: 0,
    };

    let files = fs::read_dir(&output.dir.path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let mut max_width = 0;
    for file in files{
        let f: FileEntry = FileEntry::new(file);
        if f.file_size.to_string().len() > max_width {
            max_width = f.file_size.to_string().len() as usize;
        }
        output.dir.files.push(f); 
    }
    output.column_width = max_width;

    let mut print: String;

    print = formatc!("Current Dir: ", green);
    print = format!("{}{}\n", print, &output.dir.path);

    print = format!("{}{}\n", print, formatc!("Files in Dir:",  l_blue));

    for file in &output.dir.files {
        print = format!("{}{} {:<width$} ", print, file.get_permissions(), file.file_size, width=output.column_width);
    

        if output.show_owner {
            print = format!("{}{}", print, format!("{:^width$}", file.owner, width = file.owner.len() + 2));
        }

        if output.show_last_modified {
            print = format!("{}{}", print, format!("{:^width$}", file.last_modified, width = file.last_modified.len() + 2));
        }

        if file.is_dir {
            print = format!("{}{}\n", print, formatc!(&file.file_path, d_blue));
        } else {
            print = format!("{}{}\n", print, &file.file_path);
        }
    }
    println!("{}", print);

    // output.draw();
    Ok(())
}

