pub struct CommandLineArgs {
    pub show_owner: bool,
    pub show_last_modified: bool,
    pub colour: bool,
    pub help: bool,
    pub version: bool,
}

impl CommandLineArgs {
   pub fn new(args: &[String]) -> CommandLineArgs {
        let mut show_owner = false;
        let mut show_last_modified = false;
        let mut colour = true;
        let mut help = false;
        let mut version = false;

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
                "-v" => {
                    version = true;
                }
                _ => {
                    eprintln!("Error: Unknown argument '{}'", arg);
                }
            }
        }

        CommandLineArgs { 
            show_owner, 
            show_last_modified,
            colour,
            help,
            version,
        }
    }
}

