mod file_handling;
mod string_tools;
mod console;

pub mod prelude {
    use crate::file_handling;
    use crate::string_tools;
    use crate::console;

    pub use file_handling::get_file_as_byte_vec;
    pub use file_handling::FileErrors;
    pub use string_tools::camel_case_to_snake_case;
    pub use string_tools::capitalize_first_letter;

    pub use console::Console;
}
