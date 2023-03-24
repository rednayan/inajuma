#![no_std] //removing link for rust standart library
#![no_main] //remove all entry points (Rust level)
use core::panic::PanicInfo;


static HELLO: &[u8] = b"BUFFER!";
#[no_mangle] 
pub extern "C" fn _start() -> ! { //linker looks for the entry point named '_start' by default
    let vga_buffer = 0xb8000 as *mut u8;

    for (i,&byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(1 as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { //called on panic
    loop{}
}

