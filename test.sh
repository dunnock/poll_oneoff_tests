cargo build
wasmtime -d -g --mapdir=.::./input target/wasm32-wasi/debug/poll_oneoff_tests.wasm poll_fd_read
