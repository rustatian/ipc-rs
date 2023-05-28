pub fn last_os_error(err: &std::io::Error){
    if let Some(raw_os_error) = err.raw_os_error() {
        println!("raw OS error: {raw_os_error:?}");
    } else {
        println!("not an OS error");
    }
}
