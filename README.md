# practice_rust2

RUST練習帳

/*******************************************************************/
// Linux host
cargo new mutex
cd mutex
cp ~/Downloads/vx_rust2/mutex/src/main.rs src
cargo build
ls -l target/aarch64-wrs-vxworks/debug
cp target/aarch64-wrs-vxworks/debug/mutex.vxe /media/horie/FAT32

// vxworks for raspberry Pi 4B
cmd
cd /bd0a
./mutex.vxe

/*******************************************************************/
// Build using Linux host
cargo new barrier
cd barrier
cp ~/Downloads/vx_rust2/barrier/src/main.rs src
cargo build
ls -l target/aarch64-wrs-vxworks/debug
cp target/aarch64-wrs-vxworks/debug/barrier.vxe /media/horie/FAT32

// vxworks for raspberry Pi 4B
cmd
cd /bd0a
./barrier.vxe

/*******************************************************************/
// Build using Linux host
cargo new condvar
cd condvar
cp ~/Downloads/vx_rust2/condvar/src/main.rs src
cargo build
ls -l target/aarch64-wrs-vxworks/debug
cp target/aarch64-wrs-vxworks/debug/condvar.vxe /media/horie/FAT32

// vxworks for raspberry Pi 4B
cmd
cd /bd0a
./condvar.vxe

/*******************************************************************/
// Build using Linux host
cargo new semaphore
cd semaphore
cp ~/Downloads/vx_rust2/semaphore/src/main.rs src
cp ~/Downloads/vx_rust2/semaphore/src/semaphore.rs src
cargo build
ls -l target/aarch64-wrs-vxworks/debug
cp target/aarch64-wrs-vxworks/debug/semaphore.vxe /media/horie/FAT32

// vxworks for raspberry Pi 4B
cmd
cd /bd0a
./semaphore.vxe

/*******************************************************************/
// Build using Linux host
cargo new spin
cd spin
cp ~/Downloads/vx_rust2/spin/src/main.rs src
vi Cargo.toml

[dependencies]
spin = "0.9.2"

cargo build
ls -l target/aarch64-wrs-vxworks/debug
cp target/aarch64-wrs-vxworks/debug/spin.vxe /media/horie/FAT32

// vxworks for raspberry Pi 4B
cmd
cd /bd0a
./spin.vxe

/*******************************************************************/
// Build using Linux host
cargo new write_json
cd write_json
cp ~/Downloads/vx_rust2/write_json/src/main.rs src
vi Cargo.toml

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

cargo build
ls -l target/aarch64-wrs-vxworks/debug
cp target/aarch64-wrs-vxworks/debug/write_json.vxe /media/horie/FAT32

// vxworks for raspberry Pi 4B
cmd
cd /bd0a
./write_json.vxe
more windriver.json
