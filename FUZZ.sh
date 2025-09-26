cd fuzz/tft
rm -rf in out
mkdir in && cat /dev/random | head -n 100 > in/input
cargo afl build --release && cargo afl fuzz -i in -o out target/release/tft-fuzz
