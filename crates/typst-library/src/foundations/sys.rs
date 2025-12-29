//! System-related things.

use std::io::Write;
use std::process::{Command, Stdio};
use crate::foundations::{SourceResult, Dict, Module, Scope, Version, Value, Args, func};

/// A module with system-related things.
pub fn module(inputs: Dict) -> Module {
    let typst_version = typst_utils::version();
    let version = Version::from_iter([
        typst_version.major(),
        typst_version.minor(),
        typst_version.patch(),
    ]);

    let mut scope = Scope::deduplicating();
    scope.define("version", version);
	scope.define_func::<subprocess>();
    scope.define("inputs", inputs);
    Module::new("sys", scope)
}

#[func(title = "subprocess")]
pub fn subprocess(args: &mut Args) -> SourceResult<Value> {
		let cmd = args.expect::<String>("command")?;
		let stdin_data: Option<String> = args.named("stdin")?;

		let mut command = Command::new("sh");
    command.arg("-c").arg(&cmd);

		command.stdin(Stdio::piped());
		command.stdout(Stdio::piped());

		let mut child = command
        .spawn()
        .map_err(|e| format!("Failed to start subprocess: {e}")).unwrap();

    if let Some(input) = stdin_data {
        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(input.as_bytes())
                .map_err(|e| format!("Failed to write to stdin: {e}")).unwrap();
        }
    }

		let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to read subprocess output: {e}")).unwrap();

		let stdout = String::from_utf8_lossy(&output.stdout).to_string();
		Ok(Value::Str(stdout.into()))
}
