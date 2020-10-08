#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub fn main() {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}
