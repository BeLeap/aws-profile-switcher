use std::fs;


fn main() {
    let home_dir = home::home_dir().unwrap();
    let aws_config_content = fs::read_to_string(format!("{}/.aws/config", home_dir.into_os_string().into_string().unwrap())).unwrap();

    let profiles: Vec<&str> = aws_config_content.lines().fold(vec![], |acc, line| {
        if line.starts_with("[") && line.ends_with("]") {
            [acc, vec![line]].concat()
        } else {
            acc
        }
    });

    println!("{:?}", profiles);
}
