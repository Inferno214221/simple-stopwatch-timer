use tokio::time::Duration;

pub fn format_dur(dur: Duration) -> String {
    let mut secs = dur.as_secs();
    if secs / 60 >= 1 {
        let mut mins = secs / 60;
        secs %= 60;
        if mins / 60 >= 1 {
            let hours = mins / 60;
            mins %= 60;
            format!("{}:{:0>2}:{:0>2}", hours, mins, secs)
        } else {
            format!("{}:{:0>2}", mins, secs)
        }
    } else {
        format!("{}", secs)
    }
}

pub fn format_digits(digits: u64) -> String {
    let mut display = digits.to_string();
    if display.len() > 2 {
        display.insert(display.len() - 2, ':');
    }
    if display.len() > 5 {
        display.insert(display.len() - 5, ':');
    }
    display
}

pub fn dur_from_str(input: &str) -> Option<Duration> {
    let mut split = input.split(':').collect::<Vec<_>>();
    if split.len() <= 3 {
        let mut secs = split.pop()?.parse::<u64>().ok()?;
        if !split.is_empty() {
            let mut mins = split.pop()?.parse::<u64>().ok()?;
            if !split.is_empty() {
                let hours = split.pop()?.parse::<u64>().ok()?;
                mins += hours * 60;
            }
            secs += mins * 60;
        }
        Some(Duration::from_secs(secs))
    } else {
        None
    }
}