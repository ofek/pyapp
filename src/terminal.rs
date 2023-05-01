use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

// https://github.com/sindresorhus/cli-spinners/blob/main/spinners.json
const SPINNER: &[&str] = &["∙∙∙", "●∙∙", "∙●∙", "∙∙●", "∙∙∙"];

pub fn io_progress_bar(message: String, size: u64) -> ProgressBar {
    let pb = ProgressBar::new(size);
    pb.set_message(message);
    pb.set_style(
        ProgressStyle::with_template(
            "{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes}",
        )
        .unwrap()
        .progress_chars("#>-"),
    );
    pb
}

pub fn spinner(message: String) -> ProgressBar {
    let s = ProgressBar::new(0);
    s.set_message(message);
    s.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(SPINNER),
    );
    s.enable_steady_tick(Duration::from_millis(125));
    s
}
