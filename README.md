[](This file is part of c. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/c/master/COPYRIGHT. No part of c, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.)
[](Copyright © 2016 The developers of c. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/c/master/COPYRIGHT.)

# c

c is a simple project to provide C compatibility code and commonly used idioms for Rust crates which need C wrappers or glue code. It provides a generic Cargo `build.rs` which can be symlinked into place.

c is intended to be used as a submodule under `src/` in a Cargo project

## Usage

If you have a git repository and cargo crate looking like this:-

```
	myrepo/
		.git/
		Cargo.toml
		Cargo.lock
		src/
```

Do the following:-

```bash
cd myrepo
cd src
git submodule add https://github.com/lemonrock/c.git
```

Now simply edit your `Cargo.toml` file to include `build = "src/c/build.rs"` in the `[package]` section, and create `build.h` and `build.c` in `src/`. See the [threading-support](https://github.com/lemonrock/threading-support) project for examples of these files (look in its `src/`).

At some point I'd like to add an `./install` file like that used in [.cargo](https://github.com/lemonrock/.cargo), or, possibly, even move this logic inside `.cargo` and have the `build.rs` file check for the existence of `build.c`.

## Licensing

The license for this project is MIT.

[c]: https://github.com/lemonrock/c "c GitHub page"
