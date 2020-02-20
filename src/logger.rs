use chrono::Utc;

pub fn log(service: &str, msg: &str) {
    println!("[{} - {: <12}]\t{}", Utc::now().format("%H:%M"), service, msg);
}
