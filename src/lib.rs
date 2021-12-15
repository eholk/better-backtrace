pub fn print_backtrace() {
    let mut index = 0;
    backtrace::trace(|frame| {
        backtrace::resolve_frame(frame, |sym| {
            println!("{:2}: {}", index, sym.name().expect("no symbol name"));
            if let Some(filename) = sym.filename() {
                println!("\tat {}:{}", filename.display(), sym.lineno().unwrap())
            }
            index += 1;
        });
        true
    });
}
