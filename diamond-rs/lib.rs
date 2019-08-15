#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(ptr_wrapping_offset_from)]



extern crate c2rust_bitfields;
extern crate f128;
extern crate libc;
extern crate num_traits;


#[path = "src/range.rs"]
pub mod range;
#[path = "src/symbol.rs"]
pub mod symbol;
#[path = "src/enum.rs"]
pub mod r#enum;
#[path = "src/codegen.rs"]
pub mod codegen;
#[path = "src/version.rs"]
pub mod version;
#[path = "src/backtrace.rs"]
pub mod backtrace;
#[path = "src/debug.rs"]
pub mod debug;
#[path = "src/crc.rs"]
pub mod crc;
#[path = "src/array.rs"]
pub mod array;
#[path = "src/etc.rs"]
pub mod etc;
#[path = "src/class.rs"]
pub mod class;
#[path = "src/object.rs"]
pub mod object;
#[path = "src/dump.rs"]
pub mod dump;
#[path = "src/numeric.rs"]
pub mod numeric;
#[path = "src/error.rs"]
pub mod error;
#[path = "src/eval.rs"]
pub mod eval;
#[path = "src/codedump.rs"]
pub mod codedump;
#[path = "src/gem_init_1.rs"]
pub mod gem_init_1;
#[path = "src/vm.rs"]
pub mod vm;
#[path = "src/y.tab.rs"]
pub mod y_tab;
#[path = "src/pool.rs"]
pub mod pool;
#[path = "src/compar.rs"]
pub mod compar;
#[path = "src/init.rs"]
pub mod init;
#[path = "src/kernel.rs"]
pub mod kernel;
#[path = "src/mrblib.rs"]
pub mod mrblib;
#[path = "src/gem_init_0.rs"]
pub mod gem_init_0;
#[path = "src/proc.rs"]
pub mod r#proc;
#[path = "src/ext.rs"]
pub mod ext;
#[path = "src/variable.rs"]
pub mod variable;
#[path = "src/state.rs"]
pub mod state;
#[path = "src/gc.rs"]
pub mod gc;
#[path = "src/exception.rs"]
pub mod exception;
#[path = "src/fmt_fp.rs"]
pub mod fmt_fp;
#[path = "src/hash.rs"]
pub mod hash;
#[path = "src/load.rs"]
pub mod load;
#[path = "src/print.rs"]
pub mod print;
#[path = "src/gem_init_2.rs"]
pub mod gem_init_2;

