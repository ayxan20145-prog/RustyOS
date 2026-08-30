#![no_std]
#![no_main]

use core::panic::PanicInfo;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let text = "hi";
    let fg = 0x0F;
    print(text, fg);

    loop {}
}

fn print(text: &str, fg: u8) {
    for (i, byte) in text.bytes().enumerate() {
        unsafe {
            *VGA_BUFFER.add(i * 2) = byte;
            *VGA_BUFFER.add(i * 2 + 1) = fg;
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
