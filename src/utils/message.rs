use super::emoji;

pub trait Message {
    fn message(msg: &str);

    fn success(msg: &str) {
        let msg = format!("{} {}", emoji::SPARKLES, msg);
        Self::message(&msg);
    }
    fn warning(msg: &str) {
        let msg = format!("{} {}", emoji::WARN, msg);
        Self::message(&msg);
    }
    fn info(msg: &str) {
        let msg = format!("{} {}", emoji::INFO, msg);
        Self::message(&msg);
    }
    fn error(msg: &str) {
        let msg = format!("{} {}", emoji::X, msg);
        Self::message(&msg);
    }
}

pub struct StdOut;

impl Message for StdOut {
    fn message(msg: &str) {
        println!("{msg}")
    }
}

pub struct StdErr;

impl Message for StdErr {
    fn message(msg: &str) {
        eprintln!("{msg}")
    }
}
