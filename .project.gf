[gdb]
path=./rust-gdb

[commands]
Compile peak=shell cargo b --bin peak --profile debugging
Run peak=file target/debugging/peak;run&