mod start_setup;
mod download_required_files;
mod eula_agree;
use start_setup::start_setup;
use eula_agree::eula_agree;

fn main() {
    let username = std::env::var("USERNAME").unwrap();
    start_setup(username);
}