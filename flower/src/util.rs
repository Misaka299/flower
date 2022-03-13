pub fn find_version(version: String) -> String {
    let mut result = String::new();
    for x in version.chars() {
        if x.is_numeric() {
            result.push(x);
            continue;
        }
        if x.is_whitespace() {
            break;
        }
    }
    result
}