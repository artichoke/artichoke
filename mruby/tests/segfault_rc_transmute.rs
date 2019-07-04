#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

//! This integration test checks for segfaults that stem from the improper
//! handling of `Rc` when storing the `Mrb` interpreter in the `sys::mrb_state`
//! userdata pointer as a `*mut c_void`.
//!
//! Checks for memory leaks stemming from improperly grabage collecting Ruby
//! objects created in C functions, like the call to `sys::mrb_funcall_argv`.
//!
//! This test takes out `u8::MAX + 1` clones on the `Mrb` and attempts a full
//! gc.
//!
//! If this test segfaults, we are improperly transmuting the `Rc` smart
//! pointer.

use mruby::gc::MrbGarbageCollection;
use mruby::state::State;
use mruby::sys;
use mruby::Mrb;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn segfault_rc_transmute() {
    println!("size of Rc<RefCell<State>>: {}", std::mem::size_of::<Mrb>());
    println!(
        "size of RefCell<State>: {}",
        std::mem::size_of::<RefCell<State>>()
    );
    println!("size of State: {}", std::mem::size_of::<State>());
    println!(
        "size of mrb_value: {}",
        std::mem::size_of::<sys::mrb_value>()
    );

    let interp = mruby::interpreter().expect("mrb init");
    // Increase the strong count on the Rc to 255.
    let mut interps = vec![];
    for _ in 0..254 {
        interps.push(Rc::clone(&interp));
    }
    println!("strong count = {}", Rc::strong_count(&interp));

    // create an object to collect on the mruby heap.
    let bytes = std::iter::repeat(255_u8)
        .take(1024 * 1024)
        .collect::<Vec<_>>();
    let _val = unsafe {
        sys::mrb_str_new(
            interp.borrow().mrb,
            bytes.as_ptr() as *const i8,
            bytes.len(),
        )
    };

    println!("attempting full gc");
    interp.full_gc();
    println!("full gc succeeded");

    // temporarily increase strong count to 256 and drop the reference
    let temp = Rc::clone(&interp);
    drop(temp);
    println!("strong count = {}", Rc::strong_count(&interp));

    println!("attempting full gc");
    interp.full_gc();
    // if we don't get here, we've segfaulted
    println!("full gc succeeded");

    // Increase the strong count to 256, which is beyond u8::MAX
    interps.push(Rc::clone(&interp));
    println!("strong count = {}", Rc::strong_count(&interp));

    println!("attempting full gc");
    interp.full_gc();
    // if we don't get here, we've segfaulted
    println!("full gc succeeded");
}
