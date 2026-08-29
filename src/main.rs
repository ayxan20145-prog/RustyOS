#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let text = "hello world";
    let fg = 0x04;
    print(text, fg);

    loop {}
}

fn print(text: &str, fg: u8) {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, byte) in text.bytes().enumerate() {
        unsafe {
            *vga_buffer.add(i * 2) = byte;
            *vga_buffer.add(i * 2 + 1) = fg;
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
