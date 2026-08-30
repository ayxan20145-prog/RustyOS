#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("boot.asm"));

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[unsafe(no_mangle)]
pub extern "C" fn rust_main(_magic: u32, _addr: u32) -> ! {
    clear(0x0F);
    print("hi", 0x0F);
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

fn clear(background: u8) {
    for i in 0..2000 {
        unsafe {
            *VGA_BUFFER.add(i * 2) = b' ';
            *VGA_BUFFER.add(i * 2 + 1) = background;
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print("KERNEL PANIC", 0x04);
    loop {}
}
