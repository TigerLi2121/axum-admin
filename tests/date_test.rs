use chrono::{Duration, Local};

#[test]
fn date() {
    let now = Local::now();
    println!("{} {}", now.timestamp(), now.naive_local());
    let exp = now + Duration::hours(2);
    println!("{} {}", exp.timestamp(), exp.naive_local());
}
