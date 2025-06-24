#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
	unused_assignments,
	non_upper_case_globals,
	non_snake_case,
	static_mut_refs,
    clippy::borrow_interior_mutable_const
)]

use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*}
};

pub static mut MARKED_COLORS: [bool; 256] = [false; 256];

extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    const FIGHTER_NAME: &str = "edge";
    const MARKER_FILE: &str = "cloudnair.marker";
    let mut lowest_color: i32 = -1;
    let mut marked_slots: Vec<i32> = vec![];
    for x in 0..256 {
        if let Ok(_) = std::fs::read(format!(
            "mods:/fighter/{}/motion/body/c{:02}/{}",
            FIGHTER_NAME, x, MARKER_FILE
        )) {
            unsafe {
                marked_slots.push(x as _);
                MARKED_COLORS[x as usize] = true;
                if lowest_color == -1 {
                    lowest_color = x as _ ;
                }
            }
        }
    }

	edge::install();
}

mod edge;

#[skyline::main(name = "sephiroth_cloud_nair")]
pub fn main() {
	unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                ty: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, mods_mounted);
    }
}