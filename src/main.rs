#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::Timer,
    hal::pac::TIMER0
};

fn print_e(leds: &mut [[u8; 5]; 5], timer: &mut Timer<TIMER0>, display: &mut Display) {
    for i in 0..5 {
        for j in 0..5 {
            leds[i][j] = 0;
        }
    }
    for i in (0..5).rev() {
        leds[0][i] = 1;
        display.show(timer, *leds, 100);
    }
    for i in 1..4 {
        leds[i][0] = 1;
        display.show(timer, *leds, 100);
    }
    for i in 0..5 {
        leds[4][i] = 1;
        display.show(timer, *leds, 100);
    }
    for i in 1..5 {
        leds[2][i] = 1;
        display.show(timer, *leds, 100);
    }
}

fn print_i(leds: &mut [[u8; 5]; 5], timer: &mut Timer<TIMER0>, display: &mut Display) {
    for i in 0..5 {
        for j in 0..5 {
            leds[i][j] = 0;
        }
    }
    for i in 0..5 {
        leds[0][i] = 1;
        display.show(timer, *leds, 100);
    }
    for i in 1..4 {
        leds[i][2] = 1;
        display.show(timer, *leds, 100);
    }
    for i in 0..5 {
        leds[4][i] = 1;
        display.show(timer, *leds, 100);
    }
    
}

fn print_c(leds: &mut [[u8; 5]; 5], timer: &mut Timer<TIMER0>, display: &mut Display) {
    for i in 0..5 {
        for j in 0..5 {
            leds[i][j] = 0;
        }
    }
    for i in (0..5).rev() {
        leds[0][i] = 1;
        display.show(timer, *leds, 100);
    }
    for i in 1..4 {
        leds[i][0] = 1;
        display.show(timer, *leds, 100);
    }
    for i in 0..5 {
        leds[4][i] = 1;
        display.show(timer, *leds, 100);
    }
    
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut led: [[u8; 5]; 5]= [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    
    loop{
        print_e(&mut led, &mut timer, &mut display);
        print_e(&mut led, &mut timer, &mut display);
        print_i(&mut led, &mut timer, &mut display);
        print_c(&mut led, &mut timer, &mut display);
    }
    
}