use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let input: Vec<Vec<(&str, &str)>> = input
        .split("\n\n")
        .map(|fields| {
            fields
                .split_whitespace()
                .map(|field| field.split_once(':').unwrap())
                .collect()
        })
        .collect();

    let mut valid: (u32, u32) = (0, 0);

    for passport in input {
        if vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|&field| passport.iter().any(|f| f.0 == field))
        {
            valid.0 += 1;

            if passport.iter().all(|field| match field.0 {
                "byr" | "iyr" | "eyr" => {
                    field.1.len() == 4
                        && field.1.chars().all(|c| c.is_digit(10))
                        && field.1.parse::<u16>().map_or(false, |year| match field.0 {
                            "byr" => (1920..=2002).contains(&year),
                            "iyr" => (2010..=2020).contains(&year),
                            "eyr" => (2020..=2030).contains(&year),
                            _ => false,
                        })
                }
                "hgt" => {
                    let (value, unit) = field.1.split_at(field.1.len() - 2);
                    value.parse::<u16>().map_or(false, |height| match unit {
                        "cm" => (150..=193).contains(&height),
                        "in" => (59..=76).contains(&height),
                        _ => false,
                    })
                }
                "hcl" => {
                    field.1.len() == 7
                        && field.1.chars().nth(0) == Some('#')
                        && field.1[1..].chars().all(|c| c.is_digit(16))
                }
                "ecl" => vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&field.1),
                "pid" => field.1.len() == 9 && field.1.chars().all(|c| c.is_digit(10)),
                "cid" => true,
                _ => false,
            }) {
                valid.1 += 1;
            }
        }
    }

    println!("{}\n{}", valid.0, valid.1);
}
