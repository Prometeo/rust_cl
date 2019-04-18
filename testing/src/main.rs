fn main() {
    println!("hello World!")
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n")
}

// Note: We could also make this function return a String, but that would change its behavior.
// Instead of writing to the terminal directly, it would then collect everything into a string,
// and dump all the results in one go at the end.
