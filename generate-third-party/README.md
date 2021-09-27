# generate_third_party

Generate listings of third party dependencies and their licenses for copyright
attribution in distributed Artichoke binaries.

## Usage

To generate a `THIRDPARTY` text file for all targets Artichoke supports:

```sh
bundle exec generate-third-party-text-file
```

To generate a `THIRDPARTY` text file for a single target triple:

```sh
bundle exec generate-third-party-text-file-single-target x86_64-unknown-linux-gnu
```

(or any other target)
