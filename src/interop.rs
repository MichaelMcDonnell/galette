use chips::Chip;
use errors;
use gal_builder;
use gal_builder::Equation;

use std::ffi::CStr;
use std::os::raw::c_char;

pub fn i32_to_chip(gal_type: i32) -> Chip {
    match gal_type {
        1 => Chip::GAL16V8,
        2 => Chip::GAL20V8,
        3 => Chip::GAL22V10,
        4 => Chip::GAL20RA10,
        _ => panic!("Nope")
    }
}

#[no_mangle]
pub extern "C" fn do_stuff_c(
    gal_type: i32,
    sig: *const c_char,
    eqns: *const Equation,
    num_eqns: i32,
    file_name: *const c_char,
    pin_names: *const *const c_char,
    config: *const ::jedec_writer::Config,
) -> i32 {
    let gal_type = i32_to_chip(gal_type);
    let sig = unsafe { CStr::from_ptr(sig) }.to_bytes();
    let eqns = unsafe { std::slice::from_raw_parts(eqns, num_eqns as usize) };
    let file_name = unsafe {CStr::from_ptr(file_name) };

    let num_pins = if gal_type == Chip::GAL16V8 { 20 } else { 24 };
    let cstrs = unsafe { std::slice::from_raw_parts(pin_names, num_pins) };
    let pin_names = cstrs
        .iter()
        .map(|x| unsafe { CStr::from_ptr(*x).to_str().unwrap() })
        .collect::<Vec<_>>();

    unsafe { match gal_builder::do_stuff(gal_type, sig, eqns, file_name.to_str().unwrap(), &pin_names, &(*config)) {
        Ok(()) => 0,
        Err(i) => { errors::print_error(i); i }
    } }
}