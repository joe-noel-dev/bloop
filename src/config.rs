use std::path::PathBuf;

pub struct AppConfig {
    pub root_directory: PathBuf,
    pub api_url: String,
    pub use_dummy_audio: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            root_directory: get_root_directory(),
            api_url: get_api_url(),
            use_dummy_audio: false,
        }
    }
}

impl AppConfig {
    pub fn with_root_directory(mut self, root_directory: PathBuf) -> Self {
        self.root_directory = root_directory;
        self
    }

    pub fn with_api_url(mut self, api_url: String) -> Self {
        self.api_url = api_url;
        self
    }

    pub fn with_use_dummy_audio(mut self, use_dummy_audio: bool) -> Self {
        self.use_dummy_audio = use_dummy_audio;
        self
    }
}

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
