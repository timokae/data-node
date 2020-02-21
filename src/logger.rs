use chrono::Utc;

pub fn log(service: &str, msg: &str) {
    println!(
        "[{} - {: <12}] {}",
        Utc::now().format("%H:%M:%S"),
        service,
        msg
    );
}
