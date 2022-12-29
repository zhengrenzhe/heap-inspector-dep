use env_logger::fmt::{Color, Style, StyledValue};
use log::Level;

fn colored_level<'a>(style: &'a mut Style, level: Level) -> StyledValue<'a, &'static str> {
    match level {
        Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
        Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
        Level::Info => style.set_color(Color::Green).value("INFO "),
        Level::Warn => style.set_color(Color::Yellow).value("WARN "),
        Level::Error => style.set_color(Color::Red).value("ERROR"),
    }
}

pub fn init_logger() {
    let mut builder = env_logger::builder();

    builder.format(|f, record| {
        use std::io::Write;

        let mut style = f.style();
        let level = colored_level(&mut style, record.level());

        writeln!(f, " {} {}", level, record.args(),)
    });

    builder.try_init().expect("");
}
