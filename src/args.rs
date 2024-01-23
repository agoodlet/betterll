// Define a struct to hold the parsed command-line arguments
pub struct CommandLineArgs {
    // pub file_path: String,
    pub show_owner: bool,
    pub show_last_modified: bool,
}

impl CommandLineArgs {
    // Function to parse the command-line arguments and return the struct
   pub fn new(args: &[String]) -> CommandLineArgs {
        // let mut file_path = String::new();
        let mut show_owner = false;
        let mut show_last_modified = false;

        // Skip the first argument, which is the program name itself
        let mut iter = args.iter().skip(1);

        // if the first arg doesn't start with "-" we want to take it as the file path 
        // otherwise we want to leave this as a blank string 
//         if let Some(file_path) = args.get(1) {
//             let file_path = &args[1];
//         }
// println!("{}", file_path);
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "-o" => {
                    show_owner = true;
                }
                "-m" => {
                    show_last_modified = true;
               }
                _ => {
                    eprintln!("Error: Unknown argument '{}'", arg);
                    return CommandLineArgs {
                        // file_path: file_path.to_string(),
                        show_owner: false,
                        show_last_modified: false,
                    };
                }
            }
        }

        CommandLineArgs { 
            // file_path: file_path.to_string(), 
            show_owner, 
            show_last_modified 
        }
    }
}

