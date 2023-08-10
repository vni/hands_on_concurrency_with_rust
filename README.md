# hands_on_concurrency_with_rust
Hands-On Concurrency with Rust


## Chapter 1:
### Useful tools
RUSTFLAGS="--emit asm" cargo run --target=x86_64-unknown-linux-gnu

valgrind --tool=memcheck target/debug/hello_world
valgrind --tool=massif target/debug/hello_world
valgrind --tool=cachegrind target/debug/hello_world

lldb --version
lldb target/debug/hello_world
(lldb) process launch

## Chapter 2:
rustc -C opt-level=3 src/sequential_hello_world.rs
time for i in {1..100}; do ./sequential_hello_world > /dev/null; done

rustc -C opt-level=3 src/parallel_hello_world.rs
time for i in {1..100}; do ./parallel_hello_world > /dev/null ; done

cargo afl bulid --release
cargo afl fuzz -i resources/in/ -o resources/out/ target/release/naive_interpreter

root# echo core > /proc/sys/kernel/core_pattern

valgrind --tool=memcheck target/debug/standard
valgrind --tool=cachegrind --branch-sim=yes target/debug/standard

valgrind --tool=memcheck target/debug/naive
valgrind --tool=cachegrind --branch-sim=yes target/debug/naive

perf stat --event task-clock,context-switches,page-faults,cycles,instructions,branches,branch-misses,cache-references,cache-misses target/debug/standard > /dev/null

perf stat --event task-clock,context-switches,page-faults,cycles,instructions,branches,branch-misses,cache-references,cache-misses target/debug/naive > /dev/null

cargo afl build --release
rm -rf resources/out/default/
root# echo core >/proc/sys/kernel/core_pattern
root# cd /sys/devices/system/cpu
root# echo performance | tee cpu*/cpufreq/scaling_governor
cargo afl fuzz -i resources/in/ -o resources/out/  target/release/specialized_interpreter

valgrind --tool=cachegrind --branch-sim=yes target/release/specialized
perf stat --event task-clock,context-switches,page-faults,cycles,instructions,branches,branch-misses,cache-references,cache-misses target/release/specialized