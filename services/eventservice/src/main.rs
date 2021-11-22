#![feature(array_chunks)]
mod data_manager;
mod pubsub_client;
mod types;

#[tokio::main]
async fn main() {}

#[cfg(test)]
mod tests;

#[test]
fn clean_tests() {
    assert!(std::fs::remove_dir_all("./test").is_ok());
}