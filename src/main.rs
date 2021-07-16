use std::process::Command;

fn main() -> () {
    let output = Command::new("rustc")
        .arg("-vV")
        .output()
        .expect("Error when running rustc command:");
    let stdout = std::str::from_utf8(&output.stdout).expect("NO2");
    let field = "host: ";
    let host = stdout
        .lines()
        .find(|l| l.starts_with(field))
        .map(|l| &l[field.len()..])
        .expect("Error when parsing rustc output")
        .to_string();

    println!("{}", host);
}
