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

    let results: Vec<_> = dem_strings
        .into_iter()
        .filter_map(|one_of_dem_strings| {
            let result = process_string(one_of_dem_strings);

            result
                .map(|s| {
                    if s == "ROBERT" {
                        return format!("the magnificent {}", s);
                    }
                    s
                })
                .map(|s| format!("The {} Characters of: {}", s.chars().count(), s))
                .map(|s| format!("✅: {}", s))
                .ok()
        })
        .collect();

    results.iter().for_each(|s| println!("{:?}", s));

    Ok(())
}
