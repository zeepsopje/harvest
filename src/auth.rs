use std::env;
use std::fs;
use std::io::BufReader;
use std::io::Read;
use std::net::TcpListener;
use std::process::Command;

use serde::{Serialize, Deserialize};
use serde_qs as qs;

const BASE_URL: &'static str = "https://id.getharvest.com/oauth2/authorize";

#[derive(Serialize, Deserialize)]
pub struct OAuth {
    pub access_token: String,
}

#[derive(Deserialize, Debug)]
struct QueryString {
    access_token: String,
    scope: String,
}

fn open_client_identifier() {
    dotenvy::dotenv().expect("Can't find .env");

    let client_id = env::var("CLIENT_ID")
        .expect("CLIENT_ID not set");
    let uri = format!(
        "{}?client_id={}&response_type=token",
        BASE_URL,
        client_id,
    );

    Command::new("xdg-open")
        .arg(uri)
        .spawn()
        .expect("Failed to open a browser. Make sure you've installed one.");
}

impl OAuth {
    pub fn obtain() -> anyhow::Result<Self> {
        // Listen to callback from OAuth provider
        let tcp_listener = TcpListener::bind("127.0.0.1:1111")?;

        // Start OAuth authentication process
        open_client_identifier();

        let mut buf = String::new();
        let (mut client, _) = tcp_listener
            .accept()
            .unwrap();

        client.read_to_string(&mut buf)?;

        // Parse URI from request string
        let uri = buf
            .lines()
            .collect::<Vec<_>>()[0]
            .strip_prefix("GET /?")
            .unwrap()
            .strip_suffix(" HTTP/1.1")
            .unwrap();

        let QueryString { access_token, scope } = qs::from_str(&uri)?;

        Ok(Self { access_token })
    }

    pub fn export(&self, path: &str) -> anyhow::Result<()> {
        let serialized = serde_json::to_string_pretty(self)?;

        fs::write(path, serialized)?;

        Ok(())
    }

    pub fn import(path: &str) -> Option<Self> {
        let mut f = fs::File::open(path).ok()?;
        let contents = BufReader::new(&mut f);
        let deserialized = serde_json::from_reader(contents).ok()?;

        Some(deserialized)
    }
}
