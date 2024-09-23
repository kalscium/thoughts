use log::{Level, Log, Metadata, Record};

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        match record.metadata().level() {
            Level::Error => eprintln!("\x1b[31;1merror \x1b[0m{}", record.args()),
            Level::Info  =>  println!("\x1b[36;1minfo  \x1b[0m{}", record.args()),
            Level::Warn  =>  println!("\x1b[33;1mwarn  \x1b[0m{}", record.args()),
            _ => (),
        }
    }

    fn flush(&self) {}
}
