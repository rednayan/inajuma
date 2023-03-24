#![no_std] //removing link for rust standart library
#![no_main] //remove all entry points (Rust level)
use core::panic::PanicInfo;

#[no_mangle] 
pub extern "C" fn _start() -> ! { //linker looks for the entry point named '_start' by default
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { //called on panic
    loop{}
}

