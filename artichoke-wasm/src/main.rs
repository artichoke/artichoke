#![deny(warnings, intra_doc_link_resolution_failure)]
#![deny(clippy::all, clippy::pedantic)]
#![feature(link_args)]

// mod ffi;
// mod parser;
// mod repl;

struct State {
    interp: String,
}

#[no_mangle]
pub fn artichoke_web_repl_init() -> u32 {
    let interp = match artichoke_backend::interpreter() {
        Ok(interp) => interp,
        Err(err) => {
            println!("{:?}", err);
            return 0;
        }
    };
    println!("{}", interp.borrow());
    let state = Box::new(State { interp: "a".into() });
    println!("{}", state.interp);
    Box::into_raw(state) as u32
}

#[cfg(link_args = r#"
    -s WASM=1
    -s LINKABLE=1
    -s ASSERTIONS=1
    -s ENVIRONMENT='web'
    -s EXPORTED_FUNCTIONS=["_artichoke_web_repl_init"]
    -s EXTRA_EXPORTED_RUNTIME_METHODS=["ccall","cwrap"]
"#)]
#[allow(unused_attributes)]
extern "C" {}

fn main() {}
