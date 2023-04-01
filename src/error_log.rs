pub fn error(line: usize, message: impl ToString) {
    report(line, "".into(), message.to_string());
}

fn report(line: usize, context: String, message: String) {
    eprintln!("[Line {line}] Error ({context}): {message}");
}