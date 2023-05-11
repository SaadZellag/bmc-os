mod sample_test;

use core::panic::PanicInfo;

use crate::{display::Color, print, println, set_color};

static mut CURRENT_TEST_INDEX: usize = 0;
static mut TESTS: Option<&[&dyn Testable]> = None;

static mut SUCCESS_COUNT: usize = 0;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("{}... ", core::any::type_name::<T>());
        self();
        set_color!(Color::Green, Color::Black);
        println!("[OK]");
        set_color!(Color::White, Color::Black);
    }
}

fn run_tests(tests: &[&dyn Testable], start: usize) {
    for i in start..tests.len() {
        unsafe { CURRENT_TEST_INDEX = i }
        tests[i].run();
        unsafe { SUCCESS_COUNT += 1 };
    }
}

pub fn test_runner(tests: &'static [&dyn Testable]) {
    unsafe { TESTS = Some(tests) };
    set_color!(Color::LightBlue, Color::Black);
    println!("Running {} test(s)", tests.len());
    println!();
    set_color!(Color::White, Color::Black);
    run_tests(tests, 0);
}

pub fn test_panic_handler(_info: &PanicInfo) -> ! {
    set_color!(Color::Red, Color::Black);
    println!("[FAIL]");
    set_color!(Color::White, Color::Black);
    run_tests(unsafe { TESTS.unwrap() }, unsafe { CURRENT_TEST_INDEX } + 1);

    println!();
    set_color!(Color::LightBlue, Color::Black);
    unsafe {
        println!("Passed {}/{} tests", SUCCESS_COUNT, TESTS.unwrap().len());
    }
    loop {}
}
