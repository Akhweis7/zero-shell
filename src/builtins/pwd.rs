// pwd builtin command (Member B)


function pwd()-> Result<(), ShellError> {
    let path = env::current_dir()?;
    println!("{}", path.display());
    Ok(())
}
