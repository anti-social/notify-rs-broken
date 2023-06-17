use std::{time::Duration, sync::mpsc::SyncSender};
use std::fs::create_dir;
use std::path::Path;
use std::sync::mpsc::sync_channel;
use std::thread;

use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::{DebounceEventResult, new_debouncer, DebouncedEvent};

fn main() -> anyhow::Result<()> {
    let watch_path = Path::new("data");
    if !watch_path.exists() {
        create_dir(watch_path)?;
    }

    let (tx, rx) = sync_channel(16);

    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(event) => println!("event: {:?}", event),
                Err(e) => {
                    println!("Channel was closed: {e}");
                    break;
                },
            }
        }
    });

    // let mut debouncer = new_debouncer(Duration::from_millis(500), None, move |events_result: DebounceEventResult| {
    //     match events_result {
    //         Ok(events) => {
    //             for event in events.into_iter() {
    //                 match tx.send(event) {
    //                     Ok(()) => {}
    //                     Err(e) => println!("No receivers: {e}")
    //                 }
    //             }
    //         }
    //         Err(e) => println!("watch error: {e:?}"),
    //     }
    // })?;
    // debouncer.watcher().watch(&watch_path, RecursiveMode::Recursive)?;

    // Comment following line and uncomment previous lines to fix
    watch(&watch_path, tx)?;

    println!("Watching {watch_path:?}");

    thread::sleep(Duration::from_secs(u64::MAX));

    Ok(())
}

fn watch(watch_path: &Path, tx: SyncSender<DebouncedEvent>) -> anyhow::Result<()> {
    let mut debouncer = new_debouncer(Duration::from_millis(500), None, move |events_result: DebounceEventResult| {
        match events_result {
            Ok(events) => {
                for event in events.into_iter() {
                    match tx.send(event) {
                        Ok(()) => {}
                        Err(e) => println!("No receivers: {e}")
                    }
                }
            }
            Err(e) => println!("watch error: {e:?}"),
        }
    })?;

    debouncer.watcher().watch(&watch_path, RecursiveMode::Recursive)?;

    Ok(())
}
