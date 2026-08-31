#![no_std]
#![no_main]

use core::{
    arch::global_asm,
    fmt::{self, Write},
    panic::PanicInfo,
};

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
        if byte == b'\n' {
            self.column = 0;
            self.row += 1;
            return;
        }

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
    fn write_str(&mut self, text: &str) {
        for byte in text.bytes() {
            self.write_byte(byte);
        }
    }
    fn clear(&mut self, background: u8) {
        for i in 0..2000 {
            unsafe {
                *VGA_BUFFER.add(i * 2) = b' ';
                *VGA_BUFFER.add(i * 2 + 1) = background;
            }
        }

        self.column = 0;
        self.row = 0;
    }
}

impl Write for Writer {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.write_str(text);

        Ok(())
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let mut writer = Writer::new(0x0F);
    writer.clear(0x0F);
    let logo = r#"
        ________              _____        _______________
        ___  __ \___  __________  /_____  ___  __ \_  ___/
        __  /_/ /  / / /_  ___/  __/_  / / /  / / /____ \
        _  _, _// /_/ /_(__  )/ /_ _  /_/ // /_/ /____/ /
        /_/ |_| \__,_/ /____/ \__/ _\__, / \____/ /____/
                                   /____/
    "#;
    write!(writer, "{}", logo).unwrap();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut writer = Writer::new(0x04);

    write!(writer, "KERNEL PANIC: {}", info).unwrap();

    loop {}
}
