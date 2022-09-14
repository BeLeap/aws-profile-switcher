use std::fs;

use fuzzy_finder::item::Item;


fn main() {
    let home_dir = home::home_dir().unwrap();
    let aws_config_content = fs::read_to_string(format!("{}/.aws/config", home_dir.into_os_string().into_string().unwrap())).unwrap();

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

    println!("{}", result);
}
