use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::{
    display::{get_current_color, Color},
    println, set_color,
};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    let current = get_current_color();
    set_color!(Color::Yellow, Color::Black);
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    set_color!(current);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    set_color!(Color::Yellow, Color::Black);
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
