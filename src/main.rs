use std::{fs, env, io, fs::Metadata, os::unix::prelude::PermissionsExt, path::PathBuf, error::Error, fmt, fmt::Write};

mod colors;
use colors::Colors;

mod args;
use args::CommandLineArgs;

// TODO
// - add better coloring for file entries
// - add outputs
//  - table
//  - json
//      -I'll just use serde for this, fuck writing a json serializer
// - direct ingestion thing Ben said about 
// add a max height
//  - after we hit the max height we should break out into a second column
//  - will need to get the height of the terminal
    //  - if the amount of entries is greater than the height of the terminal
    //  - start printing in the next column
    //      - might need to build in a reactive rendering system

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


// idk it's an error lol
#[derive(Debug)]
struct MetaNotFoundError {
    details: String,
}

impl MetaNotFoundError {
    fn new(msg: &str) -> Self {
        MetaNotFoundError{ details: msg.to_string() }
    } 
}

impl fmt::Display for MetaNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MetaNotFoundError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl FileEntry {
    fn new(file: PathBuf) -> Result<Self, MetaNotFoundError> {
        if file.is_symlink() {
            match fs::symlink_metadata(file.display().to_string()){
                Ok(meta) => {
                    let mut file_path: String = file.read_link().unwrap().file_name().unwrap().to_ascii_lowercase().into_string().unwrap();
                    let link: String = file.file_name().unwrap().to_ascii_lowercase().into_string().unwrap();
                    write!(file_path, " -> {}", link).unwrap();

                    let is_dir = meta.is_dir();
                    
                    Ok(
                        FileEntry {
                            file_path,
                            meta: meta.clone(),
                            file_size: meta.len(),
                            is_dir,
                            owner: "test".to_string(),
                            last_modified: "test".to_string(),
                        }
                    )
                }
                Err(_err) => {
                    //
                    let msg: String = format!("Can't resolve file meta for: {}", &file.display().to_string());
                    return Err(MetaNotFoundError::new(&msg))
                }
            }
        } else {
            match fs::metadata(file.display().to_string()){
                Ok(meta) => {
                    let file_path: String = file.file_name().unwrap().to_ascii_lowercase().into_string().unwrap();

                    let is_dir = meta.is_dir();
                    
                    Ok(
                        FileEntry {
                            file_path,
                            meta: meta.clone(),
                            file_size: meta.len(),
                            is_dir,
                            owner: "test".to_string(),
                            last_modified: "test".to_string(),
                        }
                    )
                }
                Err(_err) => {
                    //
                    let msg: String = format!("Can't resolve file meta for: {}", &file.display().to_string());
                    return Err(MetaNotFoundError::new(&msg))
                }
            }
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

impl Output {
    fn draw_colour(&self) {

        let mut print: String;
        // formatc is now using push_str under the hood so I think this is fine
        // should I just also pass in an "append" string for this macro?
        //      - so that I don't have to pass it into push_str later 
        print = formatc!("Current Dir: ", green);
        print.push_str(&self.dir.path);
        print.push_str("\n");

        print.push_str(&formatc!("Files in Dir:", l_blue));
        print.push_str("\n");

        for file in &self.dir.files {
            print.push_str(&file.get_permissions());
            // idk if I can be bothered moving this to a push and I don't know if it's worth it
            // as I would have to calculate the padding on the string before pushing it so I think that
            // it might just end up being better to format in these places and hope the performance
            // gained with the other pushes is enough
            print = format!("{} {:<width$} ", print, file.file_size, width=self.column_width);
        

            if self.show_owner {
                print = format!("{}{}", print, format!("{:^width$}", file.owner, width = file.owner.len() + 2));
            }

            if self.show_last_modified {
                print = format!("{}{}", print, format!("{:^width$}", file.last_modified, width = file.last_modified.len() + 2));
            }

            if file.is_dir {
                print.push_str(&formatc!(&file.file_path, d_blue));
                print.push_str("\n");
            } else {
                print.push_str(&file.file_path);
                print.push_str("\n");
            }
        }

        println!("{}", print);

    }

    fn draw_no_colour(&self) {

        let mut print: String = "Current Dir: ".to_string();

        print.push_str(&self.dir.path);
        print.push_str("\n");

        print.push_str("Files in Dir:");
        print.push_str("\n");

        for file in &self.dir.files {
            print.push_str(&file.get_permissions());
            // idk if I can be bothered moving this to a push and I don't know if it's worth it
            // as I would have to calculate the padding on the string before pushing it so I think that
            // it might just end up being better to format in these places and hope the performance
            // gained with the other pushes is enough
            print = format!("{} {:<width$} ", print, file.file_size, width=self.column_width);
        

            if self.show_owner {
                print = format!("{}{}", print, format!("{:^width$}", file.owner, width = file.owner.len() + 2));
            }

            if self.show_last_modified {
                print = format!("{}{}", print, format!("{:^width$}", file.last_modified, width = file.last_modified.len() + 2));
            }

            print.push_str(&file.file_path);
            print.push_str("\n");
        }

        println!("{}", print);

    }

    fn print_help() {
        let output: &str = "Usage: betterll [file path] [flags]\n\
            Flags:\n\
            -c: Disables coloured output\n\
            -o: Shows the owner of the file\n\
            -m: Shows the last modified date of the file\n\
            -h: show this help menu\n";

        print!("{}", output);
    }
}

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

    if parsed_args.help {
        Output::print_help();
        return Ok(())
    }

    let mut output = Output {
        dir: Dir {
            path: file_path,
            files: Vec::new()
        },
        show_owner: parsed_args.show_owner,
        show_last_modified: parsed_args.show_last_modified,
        column_width: 0,
    };

    //I still don't like that I'm looping through the files twice here
    let files = fs::read_dir(&output.dir.path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let mut max_width = 0;
    for file in files{
        // if we get back a MetaNotFoundError from the new function we just wanna skip it for now 
        match FileEntry::new(file){
            Ok(f) => { 
                    if f.file_size.to_string().len() > max_width {
                    max_width = f.file_size.to_string().len() as usize;
                }
                output.dir.files.push(f); 
            }
            Err(err) => {
                error!(err);
            }
        };
    }
    output.column_width = max_width;
    
    if parsed_args.colour {
        output.draw_colour();
    } else {
        output.draw_no_colour();
    }

    Ok(())
}

