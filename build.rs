// This file is part of c. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/c/master/COPYRIGHT. No part of c, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of c. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/c/master/COPYRIGHT.


extern crate gcc;
use gcc::Config;


fn main()
{
	let mut config = Config::new();
	config.flag("-Wall");
	config.flag("-Werror");
	config.file("src/build.c");
	config.compile("libbuild.a");
}
