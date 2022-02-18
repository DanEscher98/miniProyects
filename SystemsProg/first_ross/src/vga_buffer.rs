#![allow(dead_code)]
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

/* The VGA text buffer is a special memory area mapped to the VGA
 * hardware that contains the contents displayed on screen. It
 * normally consists of 25 lines that each contain 80 character
 * cells. The buffer is located at address 0xb8000. */

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

pub struct Green(pub &'static str);
impl fmt::Display for Green {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[32m")?;
        write!(f, "{}", self.0)?;
        write!(f, "\x1B[0m")?;
        Ok(())
    }
}

pub struct Red(pub &'static str);
impl fmt::Display for Red {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[31m")?;
        write!(f, "{}", self.0)?;
        write!(f, "\x1B[0m")?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
// This guarantess the correct field ordering like in a C struct
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
// Ensure that it has the same emory layout as its single filed
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

lazy_static! {
    // the spinning Mutex add safe interior mutability to our static
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
        // 1st: cast the integer 0xb8000 as an mutable raw pointer
        // 2nd: convert it to a mutable reference by dereferencing
        //      and inmediately borrowing it again (&mut)

    });
}

pub struct Writer {
    // Keeps track of the current position in the last row
    column_position: usize,
    color_code: ColorCode,
    // Explicit lifetime, specifies that the reference is valid for
    // the whole program run time (true for the VGA text buffer)
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;

                // Using the write method, instead of normal assignment,
                // we guarantee that the compiler will never optimize
                // away this write.
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_char(&mut self, character: char) {
        match character {
            '\n' => self.new_line(),
            '\t' => {
                while self.column_position % 8 != 0 {
                    self.write_byte(b' ');
                }
            }
            _ => {
                let b = encode(character).unwrap_or(0xfe); // a square character
                self.write_byte(b);
            }
        }
    }

    pub fn write_string(&mut self, message: &str) {
        for c in message.chars() {
            self.write_char(c);
        }
        // for byte in message.bytes() {
        //     match byte {
        //         // printable ASCII byte or newline
        //         0x20..=0x7e | b'\n' => self.write_byte(byte),
        //         // not part of printable ASCII range
        //         _ => self.write_byte(0xfe) // a square character
        //     }
        // }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank)
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.write_string(string);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many");
    }
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    })
}

pub fn encode(c: char) -> Option<u8> {
    let b = match c {
        '☺' => 0x01,
        '☻' => 0x02,
        '♥' => 0x03,
        '♦' => 0x04,
        '♣' => 0x05,
        '♠' => 0x06,
        '•' => 0x07,
        '◘' => 0x08,
        '○' => 0x09,
        '◙' => 0x0a,
        '♂' => 0x0b,
        '♀' => 0x0c,
        '♪' => 0x0d,
        '♫' => 0x0e,
        '☼' => 0x0f,

        '►' => 0x10,
        '◄' => 0x11,
        '↕' => 0x12,
        '‼' => 0x13,
        '¶' => 0x14,
        '§' => 0x15,
        '▬' => 0x16,
        '↨' => 0x17,
        '↑' => 0x18,
        '↓' => 0x19,
        '→' => 0x1a,
        '←' => 0x1b,
        '∟' => 0x1c,
        '↔' => 0x1d,
        '▲' => 0x1e,
        '▼' => 0x1f,

        ' '..='~' => c as u8,
        '⌂' => 0x7f,
        'Δ' => 0x7f,

        'Ç' => 0x80,
        'ü' => 0x81,
        'é' => 0x82,
        'â' => 0x83,
        'ä' => 0x84,
        'å' => 0x85,
        'ç' => 0x86,
        'ê' => 0x87,
        'ë' => 0x88,
        'è' => 0x89,
        'ï' => 0x8a,
        'î' => 0x8b,
        'ì' => 0x8c,
        'Ä' => 0x8e,
        'Å' => 0x8f,

        'É' => 0x90,
        'æ' => 0x91,
        'Æ' => 0x92,
        'ô' => 0x93,
        'ö' => 0x94,
        'ò' => 0x95,
        'û' => 0x96,
        'ù' => 0x97,
        'ÿ' => 0x98,
        'Ö' => 0x99,
        'Ü' => 0x9a,
        '¢' => 0x9b,
        '£' => 0x9c,
        '¥' => 0x9d,
        '₧' => 0x9e,
        'ƒ' => 0x9f,

        'á' => 0xa0,
        'í' => 0xa1,
        'ó' => 0xa2,
        'ú' => 0xa3,
        'ñ' => 0xa4,
        'Ñ' => 0xa5,
        'ª' => 0xa6,
        'º' => 0xa7,
        '¿' => 0xa8,
        '⌐' => 0xa9,
        '¬' => 0xaa,
        '½' => 0xab,
        '¼' => 0xac,
        '¡' => 0xad,
        '«' => 0xae,
        '»' => 0xaf,

        '░' => 0xb0,
        '▒' => 0xb1,
        '▓' => 0xb2,
        '│' => 0xb3,
        '┤' => 0xb4,
        '╡' => 0xb5,
        '╢' => 0xb6,
        '╖' => 0xb7,
        '╕' => 0xb8,
        '╣' => 0xb9,
        '║' => 0xba,
        '╗' => 0xbb,
        '╝' => 0xbc,
        '╜' => 0xbd,
        '╛' => 0xbe,
        '┐' => 0xbf,

        '└' => 0xc0,
        '┴' => 0xc1,
        '┬' => 0xc2,
        '├' => 0xc3,
        '─' => 0xc4,
        '┼' => 0xc5,
        '╞' => 0xc6,
        '╟' => 0xc7,
        '╚' => 0xc8,
        '╔' => 0xc9,
        '╩' => 0xca,
        '╦' => 0xcb,
        '╠' => 0xcc,
        '═' => 0xcd,
        '╬' => 0xce,
        '╧' => 0xcf,

        '╨' => 0xd0,
        '╤' => 0xd1,
        '╥' => 0xd2,
        '╙' => 0xd3,
        '╘' => 0xd4,
        '╒' => 0xd5,
        '╓' => 0xd6,
        '╫' => 0xd7,
        '╪' => 0xd8,
        '┘' => 0xd9,
        '┌' => 0xda,
        '█' => 0xdb,
        '▄' => 0xdc,
        '▌' => 0xdd,
        '▐' => 0xde,
        '▀' => 0xdf,

        'α' => 0xe0,
        'ß' => 0xe1,
        'β' => 0xe1,
        'Γ' => 0xe2,
        'π' => 0xe3,
        'Π' => 0xe3,
        '∏' => 0xe3,
        'Σ' => 0xe4,
        '∑' => 0xe4,
        'σ' => 0xe5,
        'µ' => 0xe6,
        'τ' => 0xe7,
        'Φ' => 0xe8,
        'Θ' => 0xe9,
        'Ω' => 0xea,
        'δ' => 0xeb,
        'ð' => 0xeb,
        '∂' => 0xeb,
        '∞' => 0xec,
        'φ' => 0xed,
        'ϕ' => 0xed,
        '𝜙' => 0xed,
        'ε' => 0xee,
        '∈' => 0xee,
        '€' => 0xee,
        '∩' => 0xef,

        '≡' => 0xf0,
        '±' => 0xf1,
        '≥' => 0xf2,
        '≤' => 0xf3,
        '⌠' => 0xf4,
        '⌡' => 0xf5,
        '÷' => 0xf6,
        '≈' => 0xf7,
        '°' => 0xf8,
        '∙' => 0xf9,
        '·' => 0xfa,
        '√' => 0xfb,
        '✓' => 0xfb,
        'ⁿ' => 0xfc,
        '²' => 0xfd,
        '■' => 0xfe,

        _ => 0x00,
    };
    if b == 0 {
        None
    } else {
        Some(b)
    }
}
