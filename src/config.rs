use std::path::PathBuf;

pub fn get_api_url() -> String {
    if let Ok(api_url) = std::env::var("BLOOP_API_URL") {
        api_url
    } else {
        "https://joe-noel-dev-bloop.fly.dev".to_string()
    }
}

pub fn get_root_directory() -> PathBuf {
    if let Ok(bloop_home) = std::env::var("BLOOP_HOME") {
        PathBuf::from(bloop_home)
    } else {
        let mut home = home::home_dir().unwrap();

        if cfg!(target_os = "ios") {
            home.push("Documents");
        }

        home.push("bloop");

        home
    }
}
