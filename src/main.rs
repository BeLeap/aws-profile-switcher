use std::fs;
use std::io::Write;
use std::convert::TryInto;

use clap::Parser;
use fuzzy_finder::item::Item;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    #[clap(short, long, value_parser, default_value = "~/.aws/credentials")]
    aws_config: String,

    #[clap(value_parser)]
    commands: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let expanded_path = shellexpand::full(&args.aws_config).unwrap().into_owned();
    let aws_config_path = std::path::Path::new(&expanded_path);
    let aws_config_content = fs::read_to_string(aws_config_path).unwrap();

    let profiles: Vec<Item<&str>> = aws_config_content.lines().fold(vec![], |acc, line| {
        if line.starts_with("[") && line.ends_with("]") {
            let profile_name = line.trim_start_matches("[").trim_end_matches("]");
            [acc, vec![Item::new(profile_name.to_string(), profile_name)]].concat()
        } else {
            acc
        }
    });
    let profile_count = profiles.len();

    let find_result = match fuzzy_finder::FuzzyFinder::find(profiles, profile_count.try_into().unwrap()) {
        Ok(result) => result,
        Err(e) => {
            std::io::stdout().flush().unwrap();
            panic!("Failed to find result: {}", e);
        }
    };
    let profile = match find_result {
        Some(result) => result,
        None => {
            panic!("Invalid result");
        }
    };
    println!();

    std::env::set_var("AWS_PROFILE", profile);

    let notify_string = format!("using AWS_PROFILE: {}", profile);
    let divider = String::from_utf8(vec![b'='; notify_string.len()]).unwrap();
    print!("\x1b[1;33m");
    println!("{}", divider);
    println!("{}", notify_string);
    println!("{}", divider);
    print!("\x1b[0m");
    std::io::stdout().flush().unwrap();

    let shell = match std::env::var("SHELL") {
        Ok(shell) => shell,
        Err(_) => "bash".to_string(),
    };
    let sub_command = args.commands.join(" ");
    let mut child = std::process::Command::new(shell)
        .args(["-c", &sub_command])
        .spawn()
        .unwrap();

    child.wait().unwrap();
}
