use std::time::Duration;
use std::fs::create_dir;
use std::path::Path;
use std::thread;

use notify::{Watcher, RecursiveMode};

fn main() -> anyhow::Result<()> {
    let watch_path = Path::new("data");
    if !watch_path.exists() {
        create_dir(watch_path)?;
    }

    let mut watcher = notify::recommended_watcher(|res| {
        match res {
           Ok(event) => println!("event: {:?}", event),
           Err(e) => println!("watch error: {:?}", e),
        }
    })?;

    watcher.watch(watch_path, RecursiveMode::Recursive)?;
    println!("Watching {watch_path:?}");

    thread::sleep(Duration::from_secs(u64::MAX));

    Ok(())
}
