# mruby-vfs

mruby-vfs is an in memory virtual unix filesystem that is used to back an mruby
interpreter in the mruby crate.

## A virtual filesystem with a fake implementation of unix file system operations.

[Documentation](https://lopopolo.github.io/ferrocarril/mruby_vfs/index.html)

mruby-vfs provides a fake implementation of unix file system-related
functionality. It abstracts away details of certain common but complex
operations (e.g., setting permissions) and makes it easier to test any file
system-related logic without having to wait for slow I/O operations or coerce
the file system into particular states.

## License

mruby-vfs is a fork of `filesystem` crate at v0.4.4 copyright Isobel Redelmeier.

<https://github.com/iredelmeier/filesystem-rs/tree/v0.4.4>
