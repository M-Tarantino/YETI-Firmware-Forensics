use colored::Colorize;

pub fn init(level: log::LevelFilter) {
    env_logger::Builder::new()
        .filter(None, level)
        .format(|buf, record| {
            use std::io::Write;
            let level_color = match record.level() {
                log::Level::Error => "[-]".red(),
                log::Level::Warn => "[!]".yellow(),
                _ => "[+]".green(),
            };
            writeln!(buf, "{} {}", level_color, record.args())
        })
        .init();
}