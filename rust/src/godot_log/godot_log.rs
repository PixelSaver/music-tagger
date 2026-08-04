use godot::prelude::*;
use log::{LevelFilter, Metadata, Record};

struct GodotLogger;

impl log::Log for GodotLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        godot_print!("[{}] {}", record.level(), record.args());
    }

    fn flush(&self) {}
}

static LOGGER: GodotLogger = GodotLogger;

pub fn init_logger() {
    let _ = log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Debug));
}