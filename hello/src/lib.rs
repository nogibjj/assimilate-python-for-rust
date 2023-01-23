/*A library for doing Marco Polo

Formula for using GitHub Copilot:
1.  Write a comment that describes what you want to do with a function
2.  Format code with cargo fmt
3.  Lint your code with cargo clippy

*/

//build a Marco Polo function that return Polo if you pass in Marco otherwise returns NOT Marco
pub fn marco_polo(input: &str) -> String {
    if input == "Marco" {
        "Polo".to_string()
    } else {
        "NOT Marco".to_string()
    }
}
