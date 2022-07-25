use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn init(debug: bool) {
    match debug {
        true => {
            debug_logger();
        }
        false => {
            logger();
        }
    }
}

fn logger() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.args()
            )
        })
        .filter(None, get_level_filter(false))
        .init();
}

fn debug_logger() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}:{} - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.file().unwrap(),
                record.line().unwrap(),
                record.args()
            )
        })
        .filter(None, get_level_filter(true))
        .init();
}

fn get_level_filter(debug_mode: bool) -> LevelFilter {
    if debug_mode {
        return LevelFilter::Debug;
    }
    LevelFilter::Info
}
