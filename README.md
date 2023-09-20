# Client-v2
cargo rustc --release --target=x86_64-pc-windows-gnu --verbose -- -Clink-args="-Wl,--subsystem,windows"

cargo build --release
