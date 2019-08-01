#![deny(warnings, intra_doc_link_resolution_failure)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::cast_possible_truncation, clippy::cast_lossless)]
#![feature(link_args)]

use artichoke_backend::eval::{Context, Eval};
use artichoke_backend::Artichoke;
use std::mem;
use std::panic::{self, AssertUnwindSafe};

mod meta;
mod string;

const REPL_FILENAME: &str = "(playground)";

struct State {
    interp: Artichoke,
    heap: string::Heap,
}

#[no_mangle]
pub fn artichoke_web_repl_init() -> u32 {
    let interp = match artichoke_backend::interpreter() {
        Ok(interp) => interp,
        Err(err) => {
            eprintln!("{:?}", err);
            panic!("Could not initialize interpreter");
        }
    };
    interp.borrow_mut().capture_output();
    interp.push_context(Context::new(REPL_FILENAME));
    let mut state = Box::new(State {
        interp,
        heap: string::Heap::default(),
    });
    let build = meta::build_info();
    println!("{}", build);
    state.heap.allocate(build);
    println!("{:?}", state.interp.borrow());
    Box::into_raw(state) as u32
}

#[no_mangle]
pub fn artichoke_string_new(state: u32) -> u32 {
    if state == 0 {
        panic!("null pointer");
    }
    let mut state = unsafe { Box::from_raw(state as *mut State) };
    let s = state.heap.allocate("".to_owned());
    mem::forget(state);
    s
}

#[no_mangle]
pub fn artichoke_string_free(state: u32, ptr: u32) {
    if state == 0 {
        panic!("null pointer");
    }
    let mut state = unsafe { Box::from_raw(state as *mut State) };
    state.heap.free(ptr);
    mem::forget(state);
}

#[no_mangle]
pub fn artichoke_string_getlen(state: u32, ptr: u32) -> u32 {
    if state == 0 {
        panic!("null pointer");
    }
    let state = unsafe { Box::from_raw(state as *mut State) };
    let len = state.heap.string_getlen(ptr);
    mem::forget(state);
    len
}

#[no_mangle]
pub fn artichoke_string_getch(state: u32, ptr: u32, idx: u32) -> u8 {
    if state == 0 {
        panic!("null pointer");
    }
    let state = unsafe { Box::from_raw(state as *mut State) };
    let ch = state.heap.string_getch(ptr, idx);
    mem::forget(state);
    ch
}

#[no_mangle]
pub fn artichoke_string_putch(state: u32, ptr: u32, ch: u8) {
    if state == 0 {
        panic!("null pointer");
    }
    let mut state = unsafe { Box::from_raw(state as *mut State) };
    state.heap.string_putch(ptr, ch);
    mem::forget(state);
}

#[no_mangle]
pub fn artichoke_eval(state: u32, ptr: u32) -> u32 {
    if state == 0 {
        panic!("null pointer");
    }
    let mut state = unsafe { Box::from_raw(state as *mut State) };
    let code = state.heap.string(ptr);
    let result = match panic::catch_unwind(AssertUnwindSafe(|| state.interp.eval(code))) {
        Ok(Ok(value)) => format!("=> {}", value.inspect()),
        Ok(Err(err)) => err.to_string(),
        Err(_) => "Panicked during eval".to_owned(),
    };
    let result = format!(
        "{}{}",
        state.interp.borrow_mut().get_and_clear_captured_output(),
        result
    );
    let s = state.heap.allocate(result);
    mem::forget(state);
    s
}

#[cfg(link_args = r#"
    -s WASM=1
    -s ASSERTIONS=1
    -s ENVIRONMENT='web'
    -s TOTAL_MEMORY=5242880
    -s EXPORTED_FUNCTIONS=["_artichoke_web_repl_init","_artichoke_string_new","_artichoke_string_free","_artichoke_string_getlen","_artichoke_string_getch","_artichoke_string_putch","_artichoke_eval"]
    -s EXTRA_EXPORTED_RUNTIME_METHODS=["ccall","cwrap"]
"#)]
#[allow(unused_attributes)]
extern "C" {}

fn main() {}
