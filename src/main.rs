#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    unsafe {
        *vga_buffer.add(0) = b'h';
        *vga_buffer.add(1) = 0x0F;
        *vga_buffer.add(2) = b'i';
        *vga_buffer.add(3) = 0x0F;
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
