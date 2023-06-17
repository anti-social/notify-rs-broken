# Reproduce strange notify-rs debouncing behavior

Run `test.sh` script together with an example `cargo run`.

There shouldn't be any messages except the `Wathing "data"` one.

Open `main.rs` and perform a commented instruction. Restart the example.
Now you should see events in a terminal.
