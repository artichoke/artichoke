use crate::convert::Convert;
use crate::extn::core::array::{backend, ArrayType};
use crate::extn::core::exception::RubyException;
use crate::value::Value;
use crate::Artichoke;

#[derive(Default, Debug, Clone, Copy)]
pub struct Empty;

impl Empty {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ArrayType for Empty {
    fn box_clone(&self) -> Box<dyn ArrayType> {
        Box::new(Self::default())
    }

    fn gc_mark(&self, interp: &Artichoke) {
        let _ = interp;
    }

    fn real_children(&self) -> usize {
        0
    }

    fn len(&self) -> usize {
        0
    }

    fn is_empty(&self) -> bool {
        true
    }

    fn get(&self, interp: &Artichoke, index: usize) -> Result<Value, Box<dyn RubyException>> {
        let _ = index;
        Ok(interp.convert(None::<Value>))
    }

    fn slice(
        &self,
        interp: &Artichoke,
        start: usize,
        len: usize,
    ) -> Result<Box<dyn ArrayType>, Box<dyn RubyException>> {
        let _ = interp;
        let _ = start;
        let _ = len;
        Ok(backend::fixed::empty())
    }

    fn set(
        &mut self,
        interp: &Artichoke,
        index: usize,
        elem: Value,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<(), Box<dyn RubyException>> {
        let alloc = if index == 0 {
            vec![backend::fixed::one(elem)]
        } else if index == 1 {
            vec![backend::fixed::two(interp.convert(None::<Value>), elem)]
        } else {
            vec![backend::fixed::hole(index), backend::fixed::one(elem)]
        };
        *realloc = Some(alloc);
        Ok(())
    }

    fn set_with_drain(
        &mut self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: Value,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<usize, Box<dyn RubyException>> {
        let _ = drain;
        let alloc = if start == 0 {
            vec![backend::fixed::one(with)]
        } else if start == 1 {
            vec![backend::fixed::two(interp.convert(None::<Value>), with)]
        } else {
            vec![backend::fixed::hole(start), backend::fixed::one(with)]
        };
        *realloc = Some(alloc);
        Ok(0)
    }

    fn set_slice(
        &mut self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: Box<dyn ArrayType>,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<usize, Box<dyn RubyException>> {
        let _ = interp;
        let _ = drain;
        let alloc = if start == 0 {
            vec![with]
        } else {
            vec![backend::fixed::hole(start), with]
        };
        *realloc = Some(alloc);
        Ok(0)
    }

    fn concat(
        &mut self,
        interp: &Artichoke,
        other: Box<dyn ArrayType>,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<(), Box<dyn RubyException>> {
        let _ = interp;
        if !other.is_empty() {
            *realloc = Some(vec![other]);
        }
        Ok(())
    }

    fn pop(
        &mut self,
        interp: &Artichoke,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<Value, Box<dyn RubyException>> {
        let _ = realloc;
        Ok(interp.convert(None::<Value>))
    }

    fn reverse(&mut self, interp: &Artichoke) -> Result<(), Box<dyn RubyException>> {
        let _ = interp;
        Ok(())
    }
}
