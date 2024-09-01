fn process_string(input: &str) -> Result<String, String> {
    // Simulate some processing that might fail
    if input.len() > 5 {
        Ok(input.to_uppercase())
    } else {
        Err("Input string is too short".to_string())
    }
}

fn main() -> Result<(), String> {
    let dem_strings = vec!["banana", "cat", "robert", "joy"];
    // println!("{:?}", dem_strings);

    for one_of_dem_strings in dem_strings {
        let result = process_string(one_of_dem_strings);

        // Using map to transform the Ok value
        let modified = result
            .map(|s| {
                if s == "ROBERT" {
                    return format!("the magnificent {}", s);
                }
                s
            })
            .map(|s| format!("The {} Characters of: {}", s.chars().count(), s))
            .map(|s| format!("✅: {}", s));

        println!("{:?}", modified);
    }

    Ok(())
}
