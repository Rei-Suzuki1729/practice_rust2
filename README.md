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