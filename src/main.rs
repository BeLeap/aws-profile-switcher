use std::fs;

use clap::Parser;
use fuzzy_finder::item::Item;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, value_parser, default_value = "~/.aws/config")]
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
            let profile_name = line.trim_start_matches("[").trim_start_matches("profile ").trim_end_matches("]");
            [acc, vec![Item::new(profile_name.to_string(), profile_name)]].concat()
        } else {
            acc
        }
    });
    let profile_count = profiles.len();

    let result = fuzzy_finder::FuzzyFinder::find(profiles, profile_count.try_into().unwrap()).unwrap().unwrap();

    let profile_env = match result {
        "default" => "",
        _ => result,
    };
    std::env::set_var("AWS_PROFILE", profile_env);

    let sub_command = args.commands.join(" ");
    let mut child = std::process::Command::new("bash")
        .args(["-c", &sub_command])
        .spawn()
        .unwrap();

    child.wait().unwrap();
}
