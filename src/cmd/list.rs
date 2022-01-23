pub fn validate_num_activities(v: &str) -> Result<(), String> {
    match v.parse::<u32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Invalid number of activities to list")),
    }
}

pub fn list_last(num: u32) {
    println!("List last {} recorded activities", num)
}

pub fn list_all() {
    println!("List all recent activities")
}