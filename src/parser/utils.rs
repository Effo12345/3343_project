pub fn exit_with_msg(msg: &str) -> ! {
    println!("ERROR: {msg}");
    std::process::exit(-1);
}

pub fn stdout_expect<T>(opt: Option<T>, msg: &str) -> T {
    match opt {
        Some(val) => val,
        None => exit_with_msg(msg)
    }
}