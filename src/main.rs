use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get latest version from github releases
    async fn get_latest_version() -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let res = client
            .get("https://api.github.com/repos/chrischtel/guesser/releases/latest")
            .header("User-Agent", "guesser")
            .send()
            .await?
            .text()
            .await?;
        let data: serde_json::Value = serde_json::from_str(&res)?;
        let version = data["tag_name"].to_string();
        Ok(version.replace('"', ""))
    }

    // compare latest version with current version
    async fn compare_versions() -> Result<(), Box<dyn std::error::Error>> {
        let latest_version = get_latest_version().await?;
        let pkg_version = env!("CARGO_PKG_VERSION");
        let v_char = 'v';
        let current_version = v_char.to_string() + pkg_version;
        if latest_version != current_version {
            println!(
                "{}",
                format!(
                    "There is a new version available: {}",
                    latest_version.green()
                )
                .bold()
            );
            println!(
                "{}",
                format!("You are currently using version {}", current_version.red()).bold()
            );
            println!(
                "{}",
                format!(
                    "You can download the latest version from {}",
                    "https://github.com/chrischtel/guesser/releases/latest"
                        .blue()
                        .underline()
                )
                .bold()
            );
        }
        Ok(())
    }

    compare_versions().await?;

    println!("Guessr, the number guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..=200);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("\n");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "{}",
                    "Please enter a number, no something else. Try again.".red()
                );
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("{}", "That is correct :)".green());
                break;
            }
        }
    }

    Ok(())
}
