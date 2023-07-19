use std::{fs, env, io, fs::Metadata};

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

    let colors: Colors = Colors::new();

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

    // print!("{:?}", cwd.meta.permissions().mode());
    println!("{}Current Dir:{} {}", colors.green, colors.clear, cwd.path);

    println!("{}Files in Dir:{}", colors.l_blue, colors.clear);
    for file in cwd.files {
        let is_dir = file.meta.is_dir();
        let color = if is_dir {&colors.purple} else {&colors.clear};
        println!("{}{}{}", color, &file.file_path, colors.clear);
        // println!("{:?}", &file.meta.is_dir());
    }

    error!("test");
    Ok(())
}
