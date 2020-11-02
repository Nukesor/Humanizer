use crate::cli::Time;
use chrono::Duration;

pub fn format_time(time: Time) {
    let mut duration = if let Some(nanos) = time.nanos {
        Duration::nanoseconds(nanos)
    } else if let Some(seconds) = time.seconds {
        Duration::seconds(seconds)
    } else {
        println!("Either specify nanos or seconds");
        std::process::exit(1);
    };

    let days = duration.num_days();
    duration = duration - Duration::days(days);

    let hours = duration.num_hours();
    duration = duration - Duration::hours(hours);

    let minutes = duration.num_minutes();
    duration = duration - Duration::minutes(minutes);

    let seconds = duration.num_seconds();

    let mut formatted = String::new();
    if days > 0 {
        formatted.push_str(&format!("{} days ", days));
    }
    formatted.push_str(&format!("{:02}:{:02}:{:02}", hours, minutes, seconds));

    print!("{}", formatted);
}
