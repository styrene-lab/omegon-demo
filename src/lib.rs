/// Smoke test project — exists to give Omegon something to read, analyze, and modify.
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_works() {
        assert_eq!(greet("Omegon"), "Hello, Omegon!");
    }
}
