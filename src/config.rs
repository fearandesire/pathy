pub fn ignored_directories() -> Vec<String> {
    vec![
        ".git".to_string(),
        ".idea".to_string(),
        "node_modules".to_string(),
        "target".to_string(),
        "dist".to_string(),
        ".vscode".to_string(),
        ".npmrc".to_string(),
        "out".to_string(),
        "build".to_string(),
        ".cache".to_string(),
        "vendor".to_string(),
        ".next".to_string(),
        ".nuxt".to_string(),
        ".jekyll-cache".to_string(),
        ".pytest_cache".to_string(),
        "logs".to_string(),
        ".yarn".to_string(),
    ]
}
