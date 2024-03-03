mod file_handling;
mod string_tools;

pub mod prelude {
    use crate::file_handling;
    use crate::string_tools;
    pub use file_handling::get_file_as_byte_vec;
    pub use file_handling::FileErrors;
    pub use string_tools::camel_case_to_snake_case;
    pub use  string_tools::capitalize_first_letter;
}
