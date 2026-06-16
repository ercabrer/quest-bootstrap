use std::env;

fn main() {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;

    println!("Current OS: {}", os);
    println!("Current ARCH: {}", arch);

    if os == 

    else if 

    else if os == "linux" {

    }
    else {
        eprintln!("Unsupported OS: {}", os);
        std::process:exit(1);
    }
}
