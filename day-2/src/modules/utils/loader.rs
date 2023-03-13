use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

pub fn init() {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&[
                "▰▱▱▱▱▱▱",
                "▰▰▱▱▱▱▱",
                "▰▰▰▱▱▱▱",
                "▰▰▰▰▱▱▱",
                "▰▰▰▰▰▱▱",
                "▰▰▰▰▰▰▱",
                "▰▰▰▰▰▰▰",
                "▰▱▱▱▱▱▱",
            ]),
    );
    pb.set_message(" _🥶 generating your cuyshort url 🥶_");
    thread::sleep(Duration::from_secs(5));
    pb.finish_with_message(" _done! 😎_");
}
