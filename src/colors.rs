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
    
#[macro_export]
macro_rules! printlnc {
    ($string: expr, $color: ident) => {
        let colors = Colors::new(); 
        println!("{}{}{}", colors.$color , $string, colors.clear);
    };
}

#[macro_export]
macro_rules! formatc {
    ($string: expr, $color: ident) => {
        {
            let colors = Colors::new();
            format!("{}{}{}", colors.$color , $string, colors.clear)
        }
    };
}

// non new line macro, might be better to check for optional arg that clears the color at a
// desired location
#[macro_export]
macro_rules! printc {
    ($string: expr, $color: ident) => {
        let colors =  Colors::new();
        print!("{}{}{}", colors.$color, $string,  colors.clear);
    };
}
    
#[macro_export]
macro_rules! error{
    ($string: expr) => {  
            printlnc!($string, red);
    }
}

#[macro_export]
macro_rules! warning {
    ($string: expr) => {
        printlnc!($string,  yellow) 
    };
}

#[macro_export]
macro_rules! info {
    ($string: expr) => {
       printlnc!($string,  green) 
    };
}

