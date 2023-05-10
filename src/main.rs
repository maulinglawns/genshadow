extern crate rpassword;
extern crate pwhash;
extern crate clap;
use pwhash::sha512_crypt;
use clap::App;

const PWLEN: usize = 12;

fn getpass() -> String {
    // Ask for password and verify
    // Return values:
    // OK: The password as string
    loop {
        let pass1 = rpassword::prompt_password_stdout("Enter your password: ")
            .unwrap();
        if pass1.len() < PWLEN {
            println!("Password is too short, use at least {PWLEN} characters.");
            continue;
        }
        let pass2 = rpassword::prompt_password_stdout("Please verify your password: ")
            .unwrap();
    
        if pass1 != pass2 {
            println!("Passwords doesn't match. Try again.");
            continue;
        }
        return pass1
    }
}

fn make_sha512_hash(pass_text: &str) {
    // Create sha512 hash
    // Arguments:
    // A string
    // Return values:
    // OK: true
    // Err: false
    if pass_text.len() < PWLEN {
        println!("Blatantly refusing to generate hash. \
        Password is too short!");
        std::process::exit(1);
    }
    println!("Your sha512 hash:\n{}", sha512_crypt::hash(pass_text).unwrap());
}

fn main() {
    // Parse arguments
    App::new("genshadow")
        .version("0.1.2")
        .about("A simple compiled command line utility for generating \
        a sha512 hash (for use in /etc/shadow).\n\
        Exit codes:\n\
        0: Success\n\
        Non-zero: Failure")
        .author("Magnus Wallin <magnuswallin@tutanota.com>")
        .get_matches();
    // Ask for password
    let newpass = getpass();
    // Generate hash & print to stdout
    make_sha512_hash(&newpass);
}
