//! RISC-V timer-related functionality

use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;
use core::sync::atomic::{AtomicUsize, Ordering};
use riscv::register::time;
/// The number of ticks per second
const TICKS_PER_SEC: usize = 100;
#[allow(dead_code)]
/// The number of milliseconds per second
const MSEC_PER_SEC: usize = 1000;
/// The number of microseconds per second
#[allow(dead_code)]
const MICRO_PER_SEC: usize = 1_000_000;

/// Number of timer interrupts observed since boot.
static TIMER_TICKS: AtomicUsize = AtomicUsize::new(0);
/// Last deadline submitted to the SBI timer.
static NEXT_TRIGGER: AtomicUsize = AtomicUsize::new(0);

/// Get the current time in ticks
pub fn get_time() -> usize {
    time::read()
}

/// get current time in milliseconds
#[allow(dead_code)]
pub fn get_time_ms() -> usize {
    get_time_us() * MSEC_PER_SEC / MICRO_PER_SEC
}

/// get current time in microseconds
#[allow(dead_code)]
pub fn get_time_us() -> usize {
    let hardware_us = time::read() * MICRO_PER_SEC / CLOCK_FREQ;
    let timer_us = TIMER_TICKS.load(Ordering::Relaxed) * (MICRO_PER_SEC / TICKS_PER_SEC);

    hardware_us.max(timer_us).max(MICRO_PER_SEC / TICKS_PER_SEC)
}

/// Set the next timer interrupt
pub fn set_next_trigger() {
    let period = CLOCK_FREQ / TICKS_PER_SEC;
    let previous = NEXT_TRIGGER.load(Ordering::Relaxed);
    let next = get_time()
        .saturating_add(period)
        .max(previous.saturating_add(period));

    NEXT_TRIGGER.store(next, Ordering::Relaxed);
    set_timer(next);
}

/// Advance the software timer after a supervisor timer interrupt.
pub fn handle_timer_interrupt() {
    TIMER_TICKS.fetch_add(1, Ordering::Relaxed);
}
