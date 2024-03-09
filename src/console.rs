/// Helpers for the console
pub struct Console;

/// https://stackoverflow.com/questions/287871/how-do-i-print-colored-text-to-the-terminal/287944#287944
/// Usage
/// ```rust
/// use rust_tools::prelude::*;
/// println!("{} This text is printed in red.", Console::FAIL);
/// println!("{} This text is printed in red.", Console::RED);
/// println!("{} This text is printed in yellow.", Console::WARNING);
/// println!("{} This text is printed in yellow.", Console::YELLOW);
/// println!("{} This text is printed in blue.", Console::OK_BLUE);
/// println!("{} This text is printed in cyan.", Console::OK_CYAN);
/// println!("{} This text is printed in green.", Console::OK_GREEN);
///
/// println!("{} This text is printed bold.", Console::BOLD);
/// println!("{} This text is underlined.", Console::UNDERLINE);
/// ```
impl Console {
    pub const FAIL          : &'static str = "\x1b[91m";
    pub const RED           : &'static str = "\x1b[91m";

    pub const WARNING       : &'static str = "\x1b[93m";
    pub const YELLOW        : &'static str = "\x1b[93m";

    pub const OK_BLUE       : &'static str = "\x1b[94m";
    pub const OK_CYAN       : &'static str = "\x1b[96m";
    pub const OK_GREEN      : &'static str = "\x1b[92m";

    pub const BOLD       : &'static str = "\x1b[1m";
    pub const UNDERLINE  : &'static str = "\x1b[4m";
}