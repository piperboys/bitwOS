#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;

mod vga;
use vga::WRITER;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");

    loop {}
}
