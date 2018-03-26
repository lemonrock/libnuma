// This file is part of libnuma. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT. No part of libnuma, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libnuma. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libnuma/master/COPYRIGHT.



use ::std::env;
use ::std::process::Command;


fn main()
{
	let cargo_manifest_folder_path = variable("CARGO_MANIFEST_DIR");
	
	// We deliberately run as much as possible outside of cargo as it makes it far easier to debug a long, complex build which has little to do with Rust.
	// Of course, this script, being shell, won't run under Windows.
	println!("{}", tool(&cargo_manifest_folder_path, "bindgen-wrapper/build-under-cargo"));
}

fn tool(cargo_manifest_folder_path: &str, program_name: &'static str) -> String
{
	let full_path = format!("{}/tools/{}", cargo_manifest_folder_path.to_owned(), program_name.to_owned());
	panic_if_process_not_successul(program_name, cargo_manifest_folder_path, Command::new(full_path))
}

fn panic_if_process_not_successul(program_name: &'static str, cargo_manifest_folder_path: &str, mut command: Command) -> String
{
	let output = command.output().unwrap_or_else(|error|
	{
		panic!("Failed to execute '{}' in '{}' error was '{}'", program_name, cargo_manifest_folder_path, error);
	});
		
	let code = output.status.code().unwrap_or_else(||
	{
		panic!("Failed to retrieve exit status from command - was it killed by a signal?");
	});

	let standard_out = String::from_utf8_lossy(&output.stdout);
	if code == 0
	{
		return standard_out.into_owned();
	}
	
	let standard_error = String::from_utf8_lossy(&output.stderr);
	panic!("Command '{}' failed with exit code '{}' (standard out was '{}'; standard error was '{}')", program_name, code, standard_out.into_owned(), standard_error.into_owned());
}

fn variable(environment_variable_name: &str) -> String
{
	env::var(environment_variable_name).unwrap()
}