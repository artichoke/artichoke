# artichoke-vfs

[![CircleCI](https://circleci.com/gh/artichoke/artichoke.svg?style=svg)](https://circleci.com/gh/artichoke/artichoke)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Virtual filesystem documentation](https://img.shields.io/badge/docs-artichoke--vfs-blue.svg)](https://artichoke.github.io/artichoke/artichoke_vfs/)

artichoke-vfs is an in memory virtual unix filesystem that is used to back an
artichoke interpreter implementation.

## A virtual filesystem with a fake implementation of unix file system operations.

artichoke-vfs provides a fake implementation of unix file system-related
functionality. It abstracts away details of certain common but complex
operations (e.g., setting permissions) and makes it easier to test any file
system-related logic without having to wait for slow I/O operations or coerce
the file system into particular states.

## License

artichoke-vfs is a fork of filesystem at
[v0.4.4](https://github.com/iredelmeier/filesystem-rs/tree/v0.4.4) copyright
Isobel Redelmeier. filesystem is licensed with the
[MIT License](https://github.com/iredelmeier/filesystem-rs/blob/v0.4.4/LICENSE).
