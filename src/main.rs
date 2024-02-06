#![no_std]
#![no_main]


mod life;
use life::*;

use cortex_m_rt::entry;
#[rustfmt::skip]
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{
        prelude::*,
        timer::Timer,
    },
};
use nanorand::{pcg64::Pcg64, Rng, SeedableRng};

use panic_halt as _;

/// Start a new run on the frame buffer `fb` from a random
/// position given by the `rng`.
fn randomize_fb(rng: &mut Pcg64, fb: &mut [[u8; 5]; 5]) {
    for r in fb.iter_mut() {
        for cell in r.iter_mut() {
            *cell = rng.generate_range(0..=1);
        }
    }
}

fn complement_fb(fb: &mut [[u8; 5]; 5]) {
    for r in fb.iter_mut(){
        for cell in r.iter_mut(){
            if *cell == 0{
                *cell = 1;
            }
            else{
                *cell = 0;
            }
        }
    }
}

#[entry]
fn init() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let buttons = board.buttons;
    let mut fb = [[0; 5]; 5];
    let mut rng = nanorand::Pcg64::new_seed(1);

    loop { 
        if done(&fb){
            randomize_fb(&mut rng, &mut fb);
        }

        if buttons.button_a.is_low().unwrap() {
            randomize_fb(&mut rng, &mut fb);
        }
        
        if buttons.button_b.is_low().unwrap() {
            complement_fb(&mut fb);
        }
        

        else {
            life(&mut fb);
        }

        display.show(&mut timer, fb, 100);
        timer.delay_ms(100u16);
    }
}
