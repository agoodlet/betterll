use std::{fs, env, io, fs::Metadata};

#[derive(Debug)]
struct Dir {
    path: String,
    files: Vec<File>,
}

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

#[allow(dead_code, unused_variables)] 
struct Colors {
    black: String,
    red: String,
    green: String,
    yellow: String,
    d_blue: String,
    purple: String,
    l_blue: String,
    white: String,
    clear: String,
}

impl Colors {
    fn new() -> Colors {
        Colors {
            black: "\x1b[30m".to_string(),
            red: "\x1b[91m".to_string(),
            green: "\x1b[92m".to_string(),
            yellow: "\x1b[93m".to_string(),
            d_blue: "\x1b[94m".to_string(),
            purple: "\x1b[95m".to_string(),
            l_blue: "\x1b[96m".to_string(),
            white: "\x1b[97m".to_string(),
            clear: "\x1b[0m".to_string(),
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

    Ok(())
}
