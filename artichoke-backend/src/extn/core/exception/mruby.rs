//! FFI glue between the Rust trampolines and the mruby C interpreter.

use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let exception_spec = class::Spec::new("Exception", None, None)?;
    class::Builder::for_spec(interp, &exception_spec).define()?;
    interp.def_class::<Exception>(exception_spec)?;

    let nomemory_spec = class::Spec::new("NoMemoryError", None, None)?;
    class::Builder::for_spec(interp, &nomemory_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<NoMemoryError>(nomemory_spec)?;

    let script_spec = class::Spec::new("ScriptError", None, None)?;
    class::Builder::for_spec(interp, &script_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<ScriptError>(script_spec)?;

    let load_spec = class::Spec::new("LoadError", None, None)?;
    class::Builder::for_spec(interp, &load_spec)
        .with_super_class::<ScriptError, _>("ScriptError")?
        .define()?;
    interp.def_class::<LoadError>(load_spec)?;

    let notimplemented_spec = class::Spec::new("NotImplementedError", None, None)?;
    class::Builder::for_spec(interp, &notimplemented_spec)
        .with_super_class::<ScriptError, _>("ScriptError")?
        .define()?;
    interp.def_class::<NotImplementedError>(notimplemented_spec)?;

    let syntax_spec = class::Spec::new("SyntaxError", None, None)?;
    class::Builder::for_spec(interp, &syntax_spec)
        .with_super_class::<ScriptError, _>("ScriptError")?
        .define()?;
    interp.def_class::<SyntaxError>(syntax_spec)?;

    let security_spec = class::Spec::new("SecurityError", None, None)?;
    class::Builder::for_spec(interp, &security_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<SecurityError>(security_spec)?;

    let signal_spec = class::Spec::new("SignalException", None, None)?;
    class::Builder::for_spec(interp, &signal_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<SignalException>(signal_spec)?;

    let interrupt_spec = class::Spec::new("Interrupt", None, None)?;
    class::Builder::for_spec(interp, &interrupt_spec)
        .with_super_class::<SignalException, _>("SignalException")?
        .define()?;
    interp.def_class::<Interrupt>(interrupt_spec)?;

    // Default for `rescue`.
    let standard_spec = class::Spec::new("StandardError", None, None)?;
    class::Builder::for_spec(interp, &standard_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<StandardError>(standard_spec)?;

    let argument_spec = class::Spec::new("ArgumentError", None, None)?;
    class::Builder::for_spec(interp, &argument_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<ArgumentError>(argument_spec)?;

    let uncaughthrow_spec = class::Spec::new("UncaughtThrowError", None, None)?;
    class::Builder::for_spec(interp, &uncaughthrow_spec)
        .with_super_class::<ArgumentError, _>("ArgumentError")?
        .define()?;
    interp.def_class::<UncaughtThrowError>(uncaughthrow_spec)?;

    let encoding_spec = class::Spec::new("EncodingError", None, None)?;
    class::Builder::for_spec(interp, &encoding_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<EncodingError>(encoding_spec)?;

    let fiber_spec = class::Spec::new("FiberError", None, None)?;
    class::Builder::for_spec(interp, &fiber_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<FiberError>(fiber_spec)?;

    let io_spec = class::Spec::new("IOError", None, None)?;
    class::Builder::for_spec(interp, &io_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<IOError>(io_spec)?;

    let eof_spec = class::Spec::new("EOFError", None, None)?;
    class::Builder::for_spec(interp, &eof_spec)
        .with_super_class::<IOError, _>("IOError")?
        .define()?;
    interp.def_class::<EOFError>(eof_spec)?;

    let index_spec = class::Spec::new("IndexError", None, None)?;
    class::Builder::for_spec(interp, &index_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<IndexError>(index_spec)?;

    let key_spec = class::Spec::new("KeyError", None, None)?;
    class::Builder::for_spec(interp, &key_spec)
        .with_super_class::<IndexError, _>("IndexError")?
        .define()?;
    interp.def_class::<KeyError>(key_spec)?;

    let stopiteration_spec = class::Spec::new("StopIteration", None, None)?;
    class::Builder::for_spec(interp, &stopiteration_spec)
        .with_super_class::<IndexError, _>("IndexError")?
        .define()?;
    interp.def_class::<StopIteration>(stopiteration_spec)?;

    let localjump_spec = class::Spec::new("LocalJumpError", None, None)?;
    class::Builder::for_spec(interp, &localjump_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<LocalJumpError>(localjump_spec)?;

    let name_spec = class::Spec::new("NameError", None, None)?;
    class::Builder::for_spec(interp, &name_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<NameError>(name_spec)?;

    let nomethod_spec = class::Spec::new("NoMethodError", None, None)?;
    class::Builder::for_spec(interp, &nomethod_spec)
        .with_super_class::<NameError, _>("NameError")?
        .define()?;
    interp.def_class::<NoMethodError>(nomethod_spec)?;

    let range_spec = class::Spec::new("RangeError", None, None)?;
    class::Builder::for_spec(interp, &range_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<RangeError>(range_spec)?;

    let floatdomain_spec = class::Spec::new("FloatDomainError", None, None)?;
    class::Builder::for_spec(interp, &floatdomain_spec)
        .with_super_class::<RangeError, _>("RangeError")?
        .define()?;
    interp.def_class::<FloatDomainError>(floatdomain_spec)?;

    let regexp_spec = class::Spec::new("RegexpError", None, None)?;
    class::Builder::for_spec(interp, &regexp_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<RegexpError>(regexp_spec)?;

    // Default `Exception` type for `raise`.
    let runtime_spec = class::Spec::new("RuntimeError", None, None)?;
    class::Builder::for_spec(interp, &runtime_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<RuntimeError>(runtime_spec)?;

    let frozen_spec = class::Spec::new("FrozenError", None, None)?;
    class::Builder::for_spec(interp, &frozen_spec)
        .with_super_class::<RuntimeError, _>("RuntimeError")?
        .define()?;
    interp.def_class::<FrozenError>(frozen_spec)?;

    let systemcall_spec = class::Spec::new("SystemCallError", None, None)?;
    class::Builder::for_spec(interp, &systemcall_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<SystemCallError>(systemcall_spec)?;

    let thread_spec = class::Spec::new("ThreadError", None, None)?;
    class::Builder::for_spec(interp, &thread_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<ThreadError>(thread_spec)?;

    let type_spec = class::Spec::new("TypeError", None, None)?;
    class::Builder::for_spec(interp, &type_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<TypeError>(type_spec)?;

    let zerodivision_spec = class::Spec::new("ZeroDivisionError", None, None)?;
    class::Builder::for_spec(interp, &zerodivision_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<ZeroDivisionError>(zerodivision_spec)?;

    let systemexit_spec = class::Spec::new("SystemExit", None, None)?;
    class::Builder::for_spec(interp, &systemexit_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<SystemExit>(systemexit_spec)?;

    let systemstack_spec = class::Spec::new("SystemStackError", None, None)?;
    class::Builder::for_spec(interp, &systemstack_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<SystemStackError>(systemstack_spec)?;

    let fatal_spec = class::Spec::new("fatal", None, None)?;
    class::Builder::for_spec(interp, &fatal_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<Fatal>(fatal_spec)?;

    let _ = interp.eval(&include_bytes!("exception.rb")[..])?;
    trace!("Patched Exception onto interpreter");
    trace!("Patched core exception hierarchy onto interpreter");
    Ok(())
}
