use std::process::Command;
fn main() {
    Command::new("tailwindcss")
        .args("-c tailwind.config.js -i input.css -o output.css".split_whitespace())
        .status()
        .unwrap();
}
