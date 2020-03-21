# Generated Stdlib Sources

This directory contains Ruby sources for standard library packages that were
extracted from a
[patched version of MRI 2.6.3](https://github.com/artichoke/ruby/tree/artichoke-vendor)
by an
[automated build process](https://github.com/artichoke/artichoke/blob/a5bb7bf7d9fa016d83e2f8ff90b989cf707cd372/artichoke-backend/build.rs#L336-L532).

The intent is to promote these generated sources to be packages included in
`artichoke-backend`'s `extn::stdlib` module. One example of a promoted package
is [`uri`](/artichoke-backend/src/extn/stdlib/uri).
