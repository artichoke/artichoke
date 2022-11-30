use artichoke_backend::extn::prelude::*;

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Container(i64);

impl HeapAllocatedData for Box<Container> {
    const RUBY_TYPE: &'static str = "Container";
}

unsafe extern "C-unwind" fn container_initialize(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let inner = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let slf = Value::from(slf);
    let inner = Value::from(inner);
    let inner = inner.try_convert_into::<i64>(&guard).unwrap_or_default();
    let container = Box::new(Container(inner));
    let result = Box::<Container>::box_into_value(container, slf, &mut guard).unwrap_or_default();
    result.into()
}

unsafe extern "C-unwind" fn container_value(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);
    let mut value = Value::from(slf);
    let result = if let Ok(data) = Box::<Container>::unbox_from_value(&mut value, &mut guard) {
        guard.interp().convert(data.0)
    } else {
        Value::nil()
    };
    result.into()
}

impl File for Container {
    type Artichoke = Artichoke;

    type Error = Error;

    fn require(interp: &mut Artichoke) -> Result<(), Self::Error> {
        let spec = class::Spec::new(
            "Container",
            qed::const_cstr_from_str!("Container\0"),
            None,
            Some(def::box_unbox_free::<Box<Self>>),
        )?;
        class::Builder::for_spec(interp, &spec)
            .value_is_rust_object()
            .add_method("initialize", container_initialize, sys::mrb_args_req(1))?
            .add_method("value", container_value, sys::mrb_args_none())?
            .define()?;
        interp.def_class::<Box<Self>>(spec)?;
        Ok(())
    }
}

#[test]
fn define_rust_backed_ruby_class() {
    let mut interp = artichoke_backend::interpreter().unwrap();
    interp.def_file_for_type::<_, Container>("container.rb").unwrap();

    interp.eval(b"require 'container'").unwrap();
    let result = interp.eval(b"Container.new(15).value").unwrap();
    let result = result.try_convert_into::<i64>(&interp).unwrap();
    assert_eq!(result, 15);
    // Ensure Rc is cloned correctly and still points to valid memory.
    let result = interp.eval(b"Container.new(105).value").unwrap();
    let result = result.try_convert_into::<i64>(&interp).unwrap();
    assert_eq!(result, 105);

    interp.close();
}
