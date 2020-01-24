#[cfg(feature = "artichoke-random")]
use rand::rngs::SmallRng;
use std::any::{Any, TypeId};
use std::borrow::Cow;
use std::collections::HashMap;
use std::ffi::c_void;
use std::io::{self, Write};
use std::ptr;

use crate::class;
use crate::convert::RustBackedValue;
use crate::eval::Context;
#[cfg(feature = "artichoke-random")]
use crate::extn::core::random::backend::rand::Rand;
use crate::fs::Filesystem;
use crate::module;
use crate::sys;
use crate::types::{Ruby, Rust};
use crate::ArtichokeError;

// NOTE: ArtichokeState assumes that it it is stored in `mrb_state->ud` wrapped in a
// [`Rc`] with type [`Artichoke`] as created by [`crate::interpreter`].
pub struct State {
    pub ctx: *mut sys::mrbc_context,
    classes: HashMap<TypeId, Box<class::Spec>>,
    modules: HashMap<TypeId, Box<module::Spec>>,
    vfs: Filesystem,
    regexp_last_evaluation_captures: usize,
    pub(crate) context_stack: Vec<Context>,
    symbol_cache: HashMap<Cow<'static, [u8]>, sys::mrb_sym>,
    captured_output: Option<Vec<u8>>,
    #[cfg(feature = "artichoke-random")]
    prng: Rand<SmallRng>,
}

impl State {
    /// Create a new [`State`] from a [`sys::mrb_state`] and
    /// [`sys::mrbc_context`] with an
    /// [in memory virtual filesystem](Filesystem).
    pub fn new(ctx: *mut sys::mrbc_context, vfs: Filesystem) -> Self {
        Self {
            ctx,
            classes: HashMap::default(),
            modules: HashMap::default(),
            vfs,
            regexp_last_evaluation_captures: 0,
            context_stack: vec![],
            symbol_cache: HashMap::default(),
            captured_output: None,
            #[cfg(feature = "artichoke-random")]
            prng: Rand::new(None),
        }
    }

    pub fn alloc<T>(
        &mut self,
        mrb: &mut sys::mrb_state,
        ptr: *mut c_void,
        into: Option<sys::mrb_value>,
    ) -> Result<sys::mrb_value, ArtichokeError>
    where
        T: RustBackedValue,
    {
        let spec = self
            .classes
            .get(&TypeId::of::<T>())
            .map(Box::as_ref)
            .ok_or_else(|| ArtichokeError::NotDefined(T::ruby_type_name().into()))?;
        if let Some(mut value) = into {
            unsafe {
                sys::mrb_sys_data_init(&mut value, ptr, spec.data_type());
            }
            Ok(value)
        } else {
            let rclass = spec
                .rclass(mrb)
                .ok_or_else(|| ArtichokeError::NotDefined(T::ruby_type_name().into()))?;
            let value = unsafe {
                let alloc = sys::mrb_data_object_alloc(mrb, rclass, ptr, spec.data_type());
                sys::mrb_sys_obj_value(alloc as *mut c_void)
            };
            Ok(value)
        }
    }

    pub fn try_get_value_from_data<T, R>(
        &mut self,
        mrb: &mut sys::mrb_state,
        value: sys::mrb_value,
    ) -> Result<*const R, ArtichokeError>
    where
        T: RustBackedValue,
    {
        let spec = self
            .classes
            .get(&TypeId::of::<T>())
            .map(Box::as_ref)
            .ok_or_else(|| ArtichokeError::NotDefined(T::ruby_type_name().into()))?;
        let rclass = spec
            .rclass(mrb)
            .ok_or_else(|| ArtichokeError::NotDefined(T::ruby_type_name().into()))?;
        // Sanity check that the RClass matches.
        let is_value_with_matching_rclass =
            ptr::eq(unsafe { sys::mrb_sys_class_of_value(mrb, value) }, rclass);
        // Sanity check that the RClass matches.
        if is_value_with_matching_rclass {
            Err(ArtichokeError::ConvertToRust {
                from: Ruby::Object,
                to: Rust::Object,
            })
        } else {
            let ptr = unsafe { sys::mrb_data_check_get_ptr(mrb, value, spec.data_type()) };
            Ok(ptr as *const R)
        }
    }

    #[cfg(feature = "artichoke-random")]
    #[must_use]
    pub fn prng(&self) -> &Rand<SmallRng> {
        &self.prng
    }

    #[cfg(feature = "artichoke-random")]
    pub fn prng_mut(&mut self) -> &mut Rand<SmallRng> {
        &mut self.prng
    }

    pub fn vfs(&self) -> &Filesystem {
        &self.vfs
    }

    pub fn vfs_mut(&mut self) -> &mut Filesystem {
        &mut self.vfs
    }

    pub fn regexp_last_evaluation_captures_mut(&mut self) -> &mut usize {
        &mut self.regexp_last_evaluation_captures
    }

    pub fn capture_output(&mut self) {
        self.captured_output = Some(Vec::default());
    }

    pub fn get_and_clear_captured_output(&mut self) -> Vec<u8> {
        self.captured_output
            .replace(Vec::default())
            .unwrap_or_default()
    }

    pub fn print(&mut self, s: &[u8]) {
        if let Some(ref mut captured_output) = self.captured_output {
            captured_output.extend_from_slice(s);
        } else {
            let _ = io::stdout().write_all(s);
            let _ = io::stdout().flush();
        }
    }

    pub fn puts(&mut self, s: &[u8]) {
        if let Some(ref mut captured_output) = self.captured_output {
            captured_output.extend_from_slice(s);
            captured_output.push(b'\n');
        } else {
            let _ = io::stdout().write_all(s);
            let _ = io::stdout().write_all(&[b'\n']);
            let _ = io::stdout().flush();
        }
    }

    /// Close a [`State`] and free underlying mruby structs and memory.
    pub fn close(&mut self, mrb: &mut sys::mrb_state) {
        unsafe {
            // Free mrb data structures
            sys::mrbc_context_free(mrb, self.ctx);
        };
    }

    /// Create a class definition bound to a Rust type `T`. Class definitions
    /// have the same lifetime as the [`State`] because the class def owns the
    /// `mrb_data_type` for the type, which must be long-lived. Class defs are
    /// stored by [`TypeId`] of `T`.
    pub fn def_class<T>(&mut self, spec: class::Spec)
    where
        T: Any,
    {
        self.classes.insert(TypeId::of::<T>(), Box::new(spec));
    }

    /// Retrieve a class definition from the state bound to Rust type `T`.
    ///
    /// This function returns `None` if type `T` has not had a class spec
    /// registered for it using [`State::def_class`].
    pub fn class_spec<T>(&self) -> Option<&class::Spec>
    where
        T: Any,
    {
        self.classes.get(&TypeId::of::<T>()).map(Box::as_ref)
    }

    /// Create a module definition bound to a Rust type `T`. Module definitions
    /// have the same lifetime as the [`State`]. Module defs are stored by
    /// [`TypeId`] of `T`.
    pub fn def_module<T>(&mut self, spec: module::Spec)
    where
        T: Any,
    {
        self.modules.insert(TypeId::of::<T>(), Box::new(spec));
    }

    /// Retrieve a module definition from the state bound to Rust type `T`.
    ///
    /// This function returns `None` if type `T` has not had a class spec
    /// registered for it using [`State::def_module`].
    pub fn module_spec<T>(&self) -> Option<&module::Spec>
    where
        T: Any,
    {
        self.modules.get(&TypeId::of::<T>()).map(Box::as_ref)
    }

    pub fn sym_intern<T>(&mut self, mrb: &mut sys::mrb_state, sym: T) -> sys::mrb_sym
    where
        T: Into<Cow<'static, [u8]>>,
    {
        let sym = sym.into();
        let ptr = sym.as_ref().as_ptr();
        let len = sym.as_ref().len();
        let interned = self
            .symbol_cache
            .entry(sym)
            .or_insert_with(|| unsafe { sys::mrb_intern_static(mrb, ptr as *const i8, len) });
        *interned
    }
}
