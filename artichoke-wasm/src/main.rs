#![deny(warnings, intra_doc_link_resolution_failure)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::cast_possible_truncation, clippy::cast_lossless)]
#![feature(link_args)]

use artichoke_backend::Artichoke;
use std::mem;

// mod parser;
// mod repl;
mod meta;
mod string;

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
    println!(
        "freeing string pointer {} from {:?}",
        ptr,
        state.interp.borrow()
    );
    state.heap.free(ptr);
    mem::forget(state);
}

#[no_mangle]
pub fn artichoke_string_getlen(state: u32, ptr: u32) -> u32 {
    if state == 0 {
        panic!("null pointer");
    }
    let state = unsafe { Box::from_raw(state as *mut State) };
    println!(
        "retrieving string pointer {} from {:?}",
        ptr,
        state.interp.borrow()
    );
    let len = state.heap.string_getlen(ptr);
    mem::forget(state);
    len
}

#[no_mangle]
pub fn artichoke_string_getch(state: u32, ptr: u32, idx: u32) -> u32 {
    if state == 0 {
        panic!("null pointer");
    }
    let state = unsafe { Box::from_raw(state as *mut State) };
    println!(
        "retrieving byte {} in string pointer {} from {:?}",
        idx,
        ptr,
        state.interp.borrow()
    );
    let ch = state.heap.string_getch(ptr, idx);
    mem::forget(state);
    ch
}

#[cfg(link_args = r#"
    -s WASM=1
    -s LINKABLE=1
    -s ASSERTIONS=1
    -s ENVIRONMENT='web'
    -s EXPORTED_FUNCTIONS=["_artichoke_web_repl_init","_artichoke_string_new","_artichoke_string_free","_artichoke_string_getlen","_artichoke_string_getch"]
    -s EXTRA_EXPORTED_RUNTIME_METHODS=["ccall","cwrap"]
"#)]
#[allow(unused_attributes)]
extern "C" {}

fn main() {}
