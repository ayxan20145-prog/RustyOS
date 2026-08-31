#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("boot.asm"));

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

struct Writer {
    column: usize,
    row: usize,
    color: u8,
}

impl Writer {
    fn new(color: u8) -> Self {
        Self {
            column: 0,
            row: 0,
            color,
        }
    }
    fn write_byte(&mut self, byte: u8) {
        if self.column >= 80 {
            self.column = 0;
            self.row += 1;
        }

        unsafe {
            let position = self.row * 80 + self.column;

            *VGA_BUFFER.add(position * 2) = byte;
            *VGA_BUFFER.add(position * 2 + 1) = self.color;
        }

        self.column += 1;
    }
    fn write_string(&mut self, text: &str) {
        for byte in text.bytes() {
            if byte == b'\n' {
                self.column = 0;
                self.row += 1;
            } else {
                self.write_byte(byte);
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(_magic: u32, _addr: u32) -> ! {
    clear(0x0F);
    let mut writer = Writer::new(0x0F);
    writer.write_string("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    loop {}
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
    let mut writer = Writer::new(0x04);

    writer.write_string("KERNEL PANIC");

    loop {}
}
