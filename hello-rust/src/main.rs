use std::process::Command;
use std::str;

// We need NodeJS installed for this to work.

fn main() {
    let javascript:&str = "console.log('Hello World');"; // The code for the hello world!

    Command::new("sh") // Create a command to test if there is a hello-world.js
            .arg("-c")
            .arg(format!("[ ! -f ./hello-world.js ] && echo \"{}\" >> hello-world.js", javascript)) // If not, make one according to the code above
            .output()
            .expect("failed to execute process");

    let helloWorldCommand = Command::new("sh") 
            .arg("-c")
            .arg("node hello-world.js") // Run the hello world
            .output()
            .expect("failed to execute process");

    println!("{}", str::from_utf8(&helloWorldCommand.stdout).unwrap().to_string()); // Print the output to console
}