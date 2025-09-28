use anyhow::{anyhow, Result};
use serde_json;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};
use tempfile::TempDir;
use tokio::time::sleep;

/// A local PocketBase instance for testing purposes
pub struct LocalPocketbase {
    temp_dir: TempDir,
    binary_path: PathBuf,
    process: Option<Child>,
    port: u16,
    admin_email: String,
    admin_password: String,
}

#[derive(Clone, Debug)]
pub struct TestUser {
    pub id: String,
    pub email: String,
    pub password: String,
    pub name: String,
}

impl TestUser {
    pub fn new(id: &str, email: &str, password: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            name: name.to_string(),
        }
    }
}

impl LocalPocketbase {
    /// Create a new LocalPocketbase instance
    pub async fn new() -> Result<Self> {
        let temp_dir = TempDir::new()?;
        let binary_path = Self::download_pocketbase(&temp_dir).await?;
        
        let port = Self::find_free_port()?;
        let admin_email = "admin@test.com".to_string();
        let admin_password = "testpassword123".to_string();

        Ok(Self {
            temp_dir,
            binary_path,
            process: None,
            port,
            admin_email,
            admin_password,
        })
    }

    /// Start the PocketBase server
    pub async fn start(&mut self) -> Result<()> {
        if self.process.is_some() {
            return Err(anyhow!("PocketBase is already running"));
        }

        let data_dir = self.temp_dir.path().join("pb_data");
        std::fs::create_dir_all(&data_dir)?;

        // Start PocketBase in background
        let mut cmd = Command::new(&self.binary_path);
        cmd.arg("serve")
           .arg("--http")
           .arg(format!("127.0.0.1:{}", self.port))
           .arg("--dir")
           .arg(&data_dir)
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());

        let process = cmd.spawn()?;
        self.process = Some(process);

        // Wait for server to be ready
        self.wait_for_ready().await?;

        // Load schema
        self.load_schema().await?;

        // Create admin user if not exists (for management operations)
        self.ensure_admin().await?;

        Ok(())
    }

    /// Stop the PocketBase server
    pub async fn stop(&mut self) -> Result<()> {
        if let Some(mut process) = self.process.take() {
            process.kill()?;
            process.wait()?;
        }
        Ok(())
    }

    /// Get the base URL for the PocketBase API
    pub fn url(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }

    /// Add a test user to the database
    pub async fn add_user(&self, user: TestUser) -> Result<()> {
        let client = reqwest::Client::new();
        
        // Create user via API
        let mut payload = HashMap::new();
        payload.insert("email", &user.email);
        payload.insert("password", &user.password);
        payload.insert("passwordConfirm", &user.password);
        payload.insert("name", &user.name);
        
        let email_visibility = "true".to_string();
        let verified = "true".to_string();
        payload.insert("emailVisibility", &email_visibility);
        payload.insert("verified", &verified);

        let url = format!("{}/api/collections/users/records", self.url());
        let response = client
            .post(&url)
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Failed to create user: {}", error_text));
        }

        Ok(())
    }

    /// Download PocketBase binary if not present
    async fn download_pocketbase(temp_dir: &TempDir) -> Result<PathBuf> {
        let pb_version = "0.27.1";
        let binary_name = if cfg!(target_os = "windows") {
            "pocketbase.exe"
        } else {
            "pocketbase"
        };
        
        let binary_path = temp_dir.path().join(binary_name);
        
        // Check if binary already exists
        if binary_path.exists() {
            return Ok(binary_path);
        }

        // Determine the correct download URL based on the platform
        let platform = if cfg!(target_os = "windows") {
            if cfg!(target_arch = "x86_64") {
                "windows_amd64"
            } else {
                "windows_arm64"
            }
        } else if cfg!(target_os = "macos") {
            if cfg!(target_arch = "x86_64") {
                "darwin_amd64"
            } else {
                "darwin_arm64"
            }
        } else {
            // Assume Linux
            if cfg!(target_arch = "x86_64") {
                "linux_amd64"
            } else if cfg!(target_arch = "aarch64") {
                "linux_arm64"
            } else {
                "linux_amd64" // fallback
            }
        };

        let download_url = format!(
            "https://github.com/pocketbase/pocketbase/releases/download/v{}/pocketbase_{}_{}.zip",
            pb_version, pb_version, platform
        );

        println!("Downloading PocketBase from: {}", download_url);

        // Download the zip file
        let client = reqwest::Client::new();
        let response = client.get(&download_url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to download PocketBase: {}", response.status()));
        }

        let zip_data = response.bytes().await?;
        
        // Extract the zip file
        let zip_path = temp_dir.path().join("pocketbase.zip");
        let mut zip_file = std::fs::File::create(&zip_path)?;
        zip_file.write_all(&zip_data)?;
        zip_file.flush()?;
        drop(zip_file);

        // Extract using the zip crate
        let file = std::fs::File::open(&zip_path)?;
        let mut archive = zip::ZipArchive::new(file)?;
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            if file.name() == binary_name {
                let mut outfile = std::fs::File::create(&binary_path)?;
                std::io::copy(&mut file, &mut outfile)?;
                
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = outfile.metadata()?.permissions();
                    perms.set_mode(0o755);
                    std::fs::set_permissions(&binary_path, perms)?;
                }
                
                break;
            }
        }

        if !binary_path.exists() {
            return Err(anyhow!("Failed to extract PocketBase binary"));
        }

        println!("PocketBase downloaded successfully to: {}", binary_path.display());
        Ok(binary_path)
    }

    /// Find a free port to use
    fn find_free_port() -> Result<u16> {
        use std::net::TcpListener;
        
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let addr = listener.local_addr()?;
        Ok(addr.port())
    }

    /// Wait for the PocketBase server to be ready
    async fn wait_for_ready(&self) -> Result<()> {
        let client = reqwest::Client::new();
        let health_url = format!("{}/api/health", self.url());
        let start_time = Instant::now();
        let timeout_duration = Duration::from_secs(30);

        while start_time.elapsed() < timeout_duration {
            if let Ok(response) = client.get(&health_url).send().await {
                if response.status().is_success() {
                    return Ok(());
                }
            }
            sleep(Duration::from_millis(100)).await;
        }

        Err(anyhow!("PocketBase server failed to start within timeout"))
    }

    /// Load the schema from the schema file
    async fn load_schema(&self) -> Result<()> {
        let schema_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("backend")
            .join("pb_schema.json");
        
        if !schema_path.exists() {
            return Err(anyhow!("Schema file not found at: {}", schema_path.display()));
        }

        let schema_content = std::fs::read_to_string(&schema_path)?;
        let collections: Vec<serde_json::Value> = serde_json::from_str(&schema_content)?;

        let client = reqwest::Client::new();

        // Import collections one by one using the collections API
        for collection in collections {
            if let Some(name) = collection.get("name").and_then(|n| n.as_str()) {
                // Skip system collections as they should already exist
                if name.starts_with('_') {
                    continue;
                }

                let create_url = format!("{}/api/collections", self.url());
                let response = client
                    .post(&create_url)
                    .json(&collection)
                    .send()
                    .await;

                match response {
                    Ok(resp) if resp.status().is_success() => {
                        println!("Successfully created collection: {}", name);
                    }
                    Ok(resp) => {
                        let error_text = resp.text().await.unwrap_or_default();
                        if error_text.contains("already exists") {
                            println!("Collection {} already exists, skipping", name);
                        } else {
                            println!("Warning: Failed to create collection {}: {}", name, error_text);
                        }
                    }
                    Err(e) => {
                        println!("Warning: Failed to create collection {}: {}", name, e);
                    }
                }
            }
        }

        Ok(())
    }

    /// Ensure admin user exists for management operations
    async fn ensure_admin(&self) -> Result<()> {
        // This is a simplified approach - in a real scenario you might need to 
        // create the admin through the initial setup process
        Ok(())
    }
}

impl Drop for LocalPocketbase {
    fn drop(&mut self) {
        if let Some(mut process) = self.process.take() {
            let _ = process.kill();
            let _ = process.wait();
        }
    }
}