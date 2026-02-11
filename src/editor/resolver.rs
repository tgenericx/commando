use std::env;

pub fn resolve() -> String {
    env::var("GIT_EDITOR")
        .or_else(|_| env::var("VISUAL"))
        .or_else(|_| env::var("EDITOR"))
        .unwrap_or_else(|_| default_editor())
}

fn default_editor() -> String {
    if cfg!(windows) {
        "notepad".to_string()
    } else {
        "vi".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn falls_back_to_platform_default() {
        let editor = resolve();
        #[cfg(windows)]
        assert_eq!(editor, "notepad");
        #[cfg(not(windows))]
        assert_eq!(editor, "vi");
    }
}
