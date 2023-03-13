use std::io;

#[path = "./modules/api.rs"]
mod api;
#[path = "./modules/utils/console.rs"]
mod console;
#[path = "./modules/utils/loader.rs"]
mod loader;

#[derive(Debug)]
struct Shorten {
    url: String,
}

fn main() {
    let input = io::stdin();
    let mut url = String::new();

    console::log("masukan url:", "");

    input.read_line(&mut url).unwrap();

    let uri = Shorten {
        url: String::from(url),
    };

    let generator = api::short_url(&uri.url);

    loader::init();

    console::log("✨ origin url", uri.url);
    console::log("🍕 new url generated:", generator);
    console::log("💖 thanks for trying cuyshort", "💖");
}
