//! Manipulate the VM `Symbol` cache.

/// Interpreters that implement [`Symbol`] expose methods for manipulating the
/// VM `Symbol` cache.
pub trait Symbol {
    /// Concrete type used to identify `Symbol`s in the VM.
    type SymbolIdentifier;

    /// Resolve a symbol from the interpreter state.
    ///
    /// Symbols are potentially interned. References to symbols occur vi an
    /// indirected value of concrete type `SymbolIdentifier`.
    fn resolve_symbol(&mut self, sym: &str) -> Self::SymbolIdentifier;
}
