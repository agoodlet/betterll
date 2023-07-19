
#[allow(dead_code, unused_variables)] 
pub struct Colors {
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub d_blue: String,
    pub purple: String,
    pub l_blue: String,
    pub white: String,
    pub clear: String,
}

impl Colors {
    pub fn new() -> Colors {
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

// TODO add macro for each color

#[macro_export]
macro_rules! error{
    ($string: expr) =>( 
        let colors = Colors::new();
        println!("{}{}{}", colors.red, $string, colors.clear);
    )
}

