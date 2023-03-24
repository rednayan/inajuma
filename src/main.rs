#![no_std]
#![no_main]
use core::{panic::PanicInfo};

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("inajuma os");
    panic!("some line did panic");
    loop {};
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print!("{}",_info);    
   loop{};
}
