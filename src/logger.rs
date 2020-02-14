use chrono::Utc;

pub fn log(msg: &str) {
    println!("[{}]\t{}", Utc::now().format("%H:%M"), msg);
}
