use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", since_the_epoch.as_secs());
    
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=assets/");
}