use chrono::Utc;

pub fn log(service: &str, msg: &str) {
    println!("[{} - {}]\t{}", Utc::now().format("%H:%M"), service, msg);
}
