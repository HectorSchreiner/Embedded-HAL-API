use avr_device::interrupt;
use core::cell::RefCell;
use arduino_hal::hal::usart::Usart0;
use arduino_hal::DefaultClock;

pub type Console = Usart0<DefaultClock>;

pub static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> =
    interrupt::Mutex::new(RefCell::new(None));

#[macro_export]
macro_rules! console_print {
    ($($t:tt)*) => {
        avr_device::interrupt::free(|cs| {
            if let Some(console) = $crate::CONSOLE.borrow(cs).borrow_mut().as_mut() {
                let _ = ufmt::uwrite!(console, $($t)*);
            }
        });
    };
}

#[macro_export]
macro_rules! console_writeln {
    ($($t:tt)*) => {
        avr_device::interrupt::free(|cs| {
            if let Some(console) = $crate::CONSOLE.borrow(cs).borrow_mut().as_mut() {
                let _ = ufmt::uwriteln!(console, $($t)*);
            }
        });
    };
}

/// Initialize the global console by providing the serial (Console) instance.
/// This avoids calling Peripherals::take() more than once.
/// 
/// **Example:**
/// ```no_run
///     let serial = arduino_hal::default_serial!(dp, pins, 57600);
///     console_init(serial);
/// ``` 
pub fn console_init(serial: Console) {
    avr_device::interrupt::free(|cs| {
        CONSOLE.borrow(cs).replace(Some(serial));
    });
}
