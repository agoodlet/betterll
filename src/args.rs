// Define a struct to hold the parsed command-line arguments
pub struct CommandLineArgs {
    pub show_owner: bool,
    pub show_last_modified: bool,
}

impl CommandLineArgs {
    // Function to parse the command-line arguments and return the struct
   pub fn new(args: &[String]) -> CommandLineArgs {
        let mut show_owner = false;
        let mut show_last_modified = false;

        // Skip the first argument, which is the program name itself
        let mut iter = args.iter().skip(1);

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                // Handle the flag "-f" (no value associated)
                "-o" => {
                    show_owner = true;
                }
                // Handle the argument "-v" with a value associated
                "-m" => {
                    show_last_modified = true;
               }
                _ => {
                    eprintln!("Error: Unknown argument '{}'", arg);
                    return CommandLineArgs {
                        show_owner: false,
                        show_last_modified: false,
                    };
                }
            }
        }

        CommandLineArgs { show_owner, show_last_modified }
    }
}

