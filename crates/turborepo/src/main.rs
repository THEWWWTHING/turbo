use std::{env::current_exe, process, process::Stdio};

use anyhow::Result;
use turborepo_lib::{Args, Payload};

fn native_run(args: Args) -> Result<i32> {
    let mut go_binary_path = current_exe()?;
    go_binary_path.pop();
    go_binary_path.pop();
    go_binary_path.pop();
    go_binary_path.push("cli");
    go_binary_path.push("turbo");

    let serialized_args = serde_json::to_string(&args)?;

    let mut command = process::Command::new(go_binary_path)
        .arg(serialized_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute turbo.");

    let exit_code = command.wait()?.code().unwrap_or(2);

    Ok(exit_code.try_into()?)
}

// This function should not expanded. Please add any logic to
// `turborepo_lib::main` instead
fn main() -> Result<()> {
    let exit_code = match turborepo_lib::main()? {
        Payload::Rust(res) => res.unwrap_or(1),
        Payload::Go(state) => native_run(*state)?,
    };

    process::exit(exit_code)
}
