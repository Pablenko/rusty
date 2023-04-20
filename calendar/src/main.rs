use chrono::{Datelike, Weekday};

fn generate_calendar(date: &chrono::NaiveDate) {
    println!("\n{}\n", date.format("%B %Y").to_string());
    println!("Mon Tue Wed Thu Fri Sat Sun\n");

    let mut next_line = String::new();
    let days_from_monday = date.weekday().num_days_from_monday() as usize;
    next_line.push_str(
        std::iter::repeat("    ")
            .take(days_from_monday)
            .collect::<String>()
            .as_str(),
    );

    for (day_idx, day) in date
        .iter_days()
        .take_while(|d| d.month() == date.month())
        .enumerate()
    {
        next_line.push_str(format!("{:<4}", day_idx + 1).as_str());
        if day.weekday() == Weekday::Sun {
            println!("{}", next_line);
            next_line.clear();
        }
    }
}

fn main() {
    println!("Please provide date in format MM-YYYY");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input = "1-".to_string() + input.trim();
    let date = chrono::NaiveDate::parse_from_str(&input, "%d-%m-%Y");
    if let Ok(date) = date {
        generate_calendar(&date);
    } else {
        println!("Invalid date");
    }
}
