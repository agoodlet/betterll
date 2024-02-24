// Define a struct to hold the parsed command-line arguments
pub struct CommandLineArgs {
    // pub file_path: String,
    pub show_owner: bool,
    pub show_last_modified: bool,
    pub colour: bool,
    pub help: bool,
}

impl CommandLineArgs {
    // Function to parse the command-line arguments and return the struct
   pub fn new(args: &[String]) -> CommandLineArgs {
        // let mut file_path = String::new();
        let mut show_owner = false;
        let mut show_last_modified = false;
        let mut colour = true;
        let mut help = false;

        // Skip the first argument, which is the program name itself
        let mut iter = args.iter().skip(1);

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "-o" => {
                    show_owner = true;
                }
                "-m" => {
                    show_last_modified = true;
                }
                "-c" =>{
                    colour = false;
                }
                "-h" => {
                    help = true;
                }
                _ => {
                    eprintln!("Error: Unknown argument '{}'", arg);
                    return CommandLineArgs {
                        // file_path: file_path.to_string(),
                        show_owner: false,
                        show_last_modified: false,
                        colour: true,
                        help: false,
                    };
                }
            }
        }

        CommandLineArgs { 
            // file_path: file_path.to_string(), 
            show_owner, 
            show_last_modified,
            colour,
            help,
        }
    }
}

