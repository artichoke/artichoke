//! FFI glue between the Rust trampolines and the mruby C interpreter.

use std::ffi::CStr;

use crate::extn::prelude::*;

const EXCEPTION_CSTR: &CStr = qed::const_cstr_from_str!("Exception\0");
const NO_MEMORY_CSTR: &CStr = qed::const_cstr_from_str!("NoMemoryError\0");
const SCRIPT_CSTR: &CStr = qed::const_cstr_from_str!("ScriptError\0");
const LOAD_CSTR: &CStr = qed::const_cstr_from_str!("LoadError\0");
const NOT_IMPLEMENTED_CSTR: &CStr = qed::const_cstr_from_str!("NotImplementedError\0");
const SYNTAX_CSTR: &CStr = qed::const_cstr_from_str!("SyntaxError\0");
const SECURITY_CSTR: &CStr = qed::const_cstr_from_str!("SecurityError\0");
const SIGNAL_CSTR: &CStr = qed::const_cstr_from_str!("SignalException\0");
const INTERRUPT_CSTR: &CStr = qed::const_cstr_from_str!("Interrupt\0");
const STANDARD_CSTR: &CStr = qed::const_cstr_from_str!("StandardError\0");
const ARGUMENT_CSTR: &CStr = qed::const_cstr_from_str!("ArgumentError\0");
const UNCAUGHT_THROW_CSTR: &CStr = qed::const_cstr_from_str!("UncaughtThrowError\0");
const ENCODING_CSTR: &CStr = qed::const_cstr_from_str!("EncodingError\0");
const FIBER_CSTR: &CStr = qed::const_cstr_from_str!("FiberError\0");
const IO_CSTR: &CStr = qed::const_cstr_from_str!("IOError\0");
const EOF_CSTR: &CStr = qed::const_cstr_from_str!("EOFError\0");
const INDEX_CSTR: &CStr = qed::const_cstr_from_str!("IndexError\0");
const KEY_CSTR: &CStr = qed::const_cstr_from_str!("KeyError\0");
const STOP_ITERATION_CSTR: &CStr = qed::const_cstr_from_str!("StopIteration\0");
const LOCAL_JUMP_CSTR: &CStr = qed::const_cstr_from_str!("LocalJumpError\0");
const NAME_CSTR: &CStr = qed::const_cstr_from_str!("NameError\0");
const NO_METHOD_CSTR: &CStr = qed::const_cstr_from_str!("NoMethodError\0");
const RANGE_CSTR: &CStr = qed::const_cstr_from_str!("RangeError\0");
const FLOAT_DOMAIN_CSTR: &CStr = qed::const_cstr_from_str!("FloatDomainError\0");
const REGEXP_CSTR: &CStr = qed::const_cstr_from_str!("RegexpError\0");
const RUNTIME_CSTR: &CStr = qed::const_cstr_from_str!("RuntimeError\0");
const FROZEN_CSTR: &CStr = qed::const_cstr_from_str!("FrozenError\0");
const SYSTEM_CALL_CSTR: &CStr = qed::const_cstr_from_str!("SystemCallError\0");
const THREAD_CSTR: &CStr = qed::const_cstr_from_str!("ThreadError\0");
const TYPE_CSTR: &CStr = qed::const_cstr_from_str!("TypeError\0");
const ZERO_DIVISION_CSTR: &CStr = qed::const_cstr_from_str!("ZeroDivisionError\0");
const SYSTEM_EXIT_CSTR: &CStr = qed::const_cstr_from_str!("SystemExit\0");
const SYSTEM_STACK_CSTR: &CStr = qed::const_cstr_from_str!("SystemStackError\0");
const FATAL_CSTR: &CStr = qed::const_cstr_from_str!("fatal\0");

static EXCEPTION_RUBY_SOURCE: &[u8] = include_bytes!("exception.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let exception_spec = class::Spec::new("Exception", EXCEPTION_CSTR, None, None)?;
    class::Builder::for_spec(interp, &exception_spec).define()?;
    interp.def_class::<Exception>(exception_spec)?;

    let nomemory_spec = class::Spec::new("NoMemoryError", NO_MEMORY_CSTR, None, None)?;
    class::Builder::for_spec(interp, &nomemory_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<NoMemoryError>(nomemory_spec)?;

    let script_spec = class::Spec::new("ScriptError", SCRIPT_CSTR, None, None)?;
    class::Builder::for_spec(interp, &script_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<ScriptError>(script_spec)?;

    let load_spec = class::Spec::new("LoadError", LOAD_CSTR, None, None)?;
    class::Builder::for_spec(interp, &load_spec)
        .with_super_class::<ScriptError, _>("ScriptError")?
        .define()?;
    interp.def_class::<LoadError>(load_spec)?;

    let notimplemented_spec = class::Spec::new("NotImplementedError", NOT_IMPLEMENTED_CSTR, None, None)?;
    class::Builder::for_spec(interp, &notimplemented_spec)
        .with_super_class::<ScriptError, _>("ScriptError")?
        .define()?;
    interp.def_class::<NotImplementedError>(notimplemented_spec)?;

    let syntax_spec = class::Spec::new("SyntaxError", SYNTAX_CSTR, None, None)?;
    class::Builder::for_spec(interp, &syntax_spec)
        .with_super_class::<ScriptError, _>("ScriptError")?
        .define()?;
    interp.def_class::<SyntaxError>(syntax_spec)?;

    let security_spec = class::Spec::new("SecurityError", SECURITY_CSTR, None, None)?;
    class::Builder::for_spec(interp, &security_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<SecurityError>(security_spec)?;

    let signal_spec = class::Spec::new("SignalException", SIGNAL_CSTR, None, None)?;
    class::Builder::for_spec(interp, &signal_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<SignalException>(signal_spec)?;

    let interrupt_spec = class::Spec::new("Interrupt", INTERRUPT_CSTR, None, None)?;
    class::Builder::for_spec(interp, &interrupt_spec)
        .with_super_class::<SignalException, _>("SignalException")?
        .define()?;
    interp.def_class::<Interrupt>(interrupt_spec)?;

    // Default for `rescue`.
    let standard_spec = class::Spec::new("StandardError", STANDARD_CSTR, None, None)?;
    class::Builder::for_spec(interp, &standard_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<StandardError>(standard_spec)?;

    let argument_spec = class::Spec::new("ArgumentError", ARGUMENT_CSTR, None, None)?;
    class::Builder::for_spec(interp, &argument_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<ArgumentError>(argument_spec)?;

    let uncaughthrow_spec = class::Spec::new("UncaughtThrowError", UNCAUGHT_THROW_CSTR, None, None)?;
    class::Builder::for_spec(interp, &uncaughthrow_spec)
        .with_super_class::<ArgumentError, _>("ArgumentError")?
        .define()?;
    interp.def_class::<UncaughtThrowError>(uncaughthrow_spec)?;

    let encoding_spec = class::Spec::new("EncodingError", ENCODING_CSTR, None, None)?;
    class::Builder::for_spec(interp, &encoding_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<EncodingError>(encoding_spec)?;

    let fiber_spec = class::Spec::new("FiberError", FIBER_CSTR, None, None)?;
    class::Builder::for_spec(interp, &fiber_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<FiberError>(fiber_spec)?;

    let io_spec = class::Spec::new("IOError", IO_CSTR, None, None)?;
    class::Builder::for_spec(interp, &io_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<IOError>(io_spec)?;

    let eof_spec = class::Spec::new("EOFError", EOF_CSTR, None, None)?;
    class::Builder::for_spec(interp, &eof_spec)
        .with_super_class::<IOError, _>("IOError")?
        .define()?;
    interp.def_class::<EOFError>(eof_spec)?;

    let index_spec = class::Spec::new("IndexError", INDEX_CSTR, None, None)?;
    class::Builder::for_spec(interp, &index_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<IndexError>(index_spec)?;

    let key_spec = class::Spec::new("KeyError", KEY_CSTR, None, None)?;
    class::Builder::for_spec(interp, &key_spec)
        .with_super_class::<IndexError, _>("IndexError")?
        .define()?;
    interp.def_class::<KeyError>(key_spec)?;

    let stopiteration_spec = class::Spec::new("StopIteration", STOP_ITERATION_CSTR, None, None)?;
    class::Builder::for_spec(interp, &stopiteration_spec)
        .with_super_class::<IndexError, _>("IndexError")?
        .define()?;
    interp.def_class::<StopIteration>(stopiteration_spec)?;

    let localjump_spec = class::Spec::new("LocalJumpError", LOCAL_JUMP_CSTR, None, None)?;
    class::Builder::for_spec(interp, &localjump_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<LocalJumpError>(localjump_spec)?;

    let name_spec = class::Spec::new("NameError", NAME_CSTR, None, None)?;
    class::Builder::for_spec(interp, &name_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<NameError>(name_spec)?;

    let nomethod_spec = class::Spec::new("NoMethodError", NO_METHOD_CSTR, None, None)?;
    class::Builder::for_spec(interp, &nomethod_spec)
        .with_super_class::<NameError, _>("NameError")?
        .define()?;
    interp.def_class::<NoMethodError>(nomethod_spec)?;

    let range_spec = class::Spec::new("RangeError", RANGE_CSTR, None, None)?;
    class::Builder::for_spec(interp, &range_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<RangeError>(range_spec)?;

    let floatdomain_spec = class::Spec::new("FloatDomainError", FLOAT_DOMAIN_CSTR, None, None)?;
    class::Builder::for_spec(interp, &floatdomain_spec)
        .with_super_class::<RangeError, _>("RangeError")?
        .define()?;
    interp.def_class::<FloatDomainError>(floatdomain_spec)?;

    let regexp_spec = class::Spec::new("RegexpError", REGEXP_CSTR, None, None)?;
    class::Builder::for_spec(interp, &regexp_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<RegexpError>(regexp_spec)?;

    // Default `Exception` type for `raise`.
    let runtime_spec = class::Spec::new("RuntimeError", RUNTIME_CSTR, None, None)?;
    class::Builder::for_spec(interp, &runtime_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<RuntimeError>(runtime_spec)?;

    let frozen_spec = class::Spec::new("FrozenError", FROZEN_CSTR, None, None)?;
    class::Builder::for_spec(interp, &frozen_spec)
        .with_super_class::<RuntimeError, _>("RuntimeError")?
        .define()?;
    interp.def_class::<FrozenError>(frozen_spec)?;

    let systemcall_spec = class::Spec::new("SystemCallError", SYSTEM_CALL_CSTR, None, None)?;
    class::Builder::for_spec(interp, &systemcall_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<SystemCallError>(systemcall_spec)?;

    let thread_spec = class::Spec::new("ThreadError", THREAD_CSTR, None, None)?;
    class::Builder::for_spec(interp, &thread_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<ThreadError>(thread_spec)?;

    let type_spec = class::Spec::new("TypeError", TYPE_CSTR, None, None)?;
    class::Builder::for_spec(interp, &type_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<TypeError>(type_spec)?;

    let zerodivision_spec = class::Spec::new("ZeroDivisionError", ZERO_DIVISION_CSTR, None, None)?;
    class::Builder::for_spec(interp, &zerodivision_spec)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<ZeroDivisionError>(zerodivision_spec)?;

    let systemexit_spec = class::Spec::new("SystemExit", SYSTEM_EXIT_CSTR, None, None)?;
    class::Builder::for_spec(interp, &systemexit_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<SystemExit>(systemexit_spec)?;

    let systemstack_spec = class::Spec::new("SystemStackError", SYSTEM_STACK_CSTR, None, None)?;
    class::Builder::for_spec(interp, &systemstack_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<SystemStackError>(systemstack_spec)?;

    let fatal_spec = class::Spec::new("fatal", FATAL_CSTR, None, None)?;
    class::Builder::for_spec(interp, &fatal_spec)
        .with_super_class::<Exception, _>("Exception")?
        .define()?;
    interp.def_class::<Fatal>(fatal_spec)?;

    interp.eval(EXCEPTION_RUBY_SOURCE)?;

    Ok(())
}
