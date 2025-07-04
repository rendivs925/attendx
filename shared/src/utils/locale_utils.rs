use log::warn;
use serde_json::Value;
use std::{collections::HashMap, fmt, fs, path::Path};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lang {
    En,
    Id,
    De,
    Ja,
}

impl Lang {
    pub fn from_code(code: &str) -> Self {
        match code.to_ascii_lowercase().as_str() {
            "id" => Self::Id,
            "de" => Self::De,
            "ja" => Self::Ja,
            _ => Self::En,
        }
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Lang::En => "en",
                Lang::Id => "id",
                Lang::De => "de",
                Lang::Ja => "ja",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Namespace {
    Validation,
    User,
    Auth,
    Common,
    Organization,
    Attendance,
}

impl fmt::Display for Namespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Namespace::Validation => "validation",
                Namespace::User => "user",
                Namespace::Auth => "auth",
                Namespace::Common => "common",
                Namespace::Organization => "organization",
                Namespace::Attendance => "attendance",
            }
        )
    }
}

#[derive(Debug, Error)]
pub enum MessageError {
    #[error("Missing namespace '{0}'")]
    MissingNamespace(Namespace),

    #[error("Missing key '{path}' in namespace '{namespace}'")]
    MissingKey { namespace: Namespace, path: String },

    #[error("Expected string at '{path}' in namespace '{namespace}', got non-string")]
    InvalidType { namespace: Namespace, path: String },
}

fn default_namespaces() -> [Namespace; 6] {
    [
        Namespace::User,
        Namespace::Validation,
        Namespace::Auth,
        Namespace::Common,
        Namespace::Organization,
        Namespace::Attendance,
    ]
}

fn load_fs(lang: Lang, ns: &Namespace) -> Value {
    let path = Path::new("./shared/locales")
        .join(lang.to_string())
        .join(format!("{ns}.json"));
    match fs::read_to_string(&path) {
        Ok(text) => serde_json::from_str(&text).unwrap_or_else(|e| {
            eprintln!("[ERROR] JSON parse failed at {:?}: {e}", path);
            Value::Null
        }),
        Err(e) => {
            eprintln!("[ERROR] File read failed at {:?}: {e}", path);
            Value::Null
        }
    }
}

async fn load_http(lang: Lang, ns: &Namespace) -> Value {
    let url = format!("http://localhost:3000/locales/{lang}/{ns}.json");
    match gloo_net::http::Request::get(&url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(text) => serde_json::from_str(&text).unwrap_or(Value::Null),
            Err(e) => {
                eprintln!("[ERROR] HTTP body read failed: {e}");
                Value::Null
            }
        },
        Err(e) => {
            eprintln!("[ERROR] HTTP request failed: {e}");
            Value::Null
        }
    }
}

pub trait MessageLookup: Sync + Send {
    fn namespaces(&self) -> &HashMap<Namespace, Value>;

    fn get(&self, ns: &Namespace, path: &str) -> Option<&Value> {
        let mut current = self.namespaces().get(ns)?;
        for key in path.split('.') {
            current = current.get(key)?;
        }
        Some(current)
    }

    fn get_str(&self, ns: Namespace, path: &str) -> Result<String, MessageError> {
        let val = self
            .get(&ns, path)
            .ok_or_else(|| MessageError::MissingKey {
                namespace: ns,
                path: path.to_string(),
            })?;

        val.as_str()
            .map(str::to_string)
            .ok_or_else(|| MessageError::InvalidType {
                namespace: ns,
                path: path.to_string(),
            })
    }

    fn get_message(&self, ns: Namespace, path: &str) -> String {
        match self.get_str(ns, path) {
            Ok(msg) => msg,
            Err(e) => {
                warn!("Message fetch failed: {}.{}: {e}", ns, path);
                format!("Error: missing message for {}.{}", ns, path)
            }
        }
    }
}

#[derive(Debug)]
pub struct Messages {
    namespaces: HashMap<Namespace, Value>,
}

impl Messages {
    pub fn new(lang: Lang) -> Self {
        let mut namespaces = HashMap::new();

        for ns in default_namespaces() {
            let json = load_fs(lang, &ns);
            assert!(
                json.is_object(),
                "Invalid or missing messages for '{ns}' in '{lang}'"
            );
            namespaces.insert(ns, json);
        }

        Self { namespaces }
    }
}

impl MessageLookup for Messages {
    fn namespaces(&self) -> &HashMap<Namespace, Value> {
        &self.namespaces
    }
}

#[derive(Debug, Clone)]
pub struct MessagesHttp {
    namespaces: HashMap<Namespace, Value>,
}

impl MessagesHttp {
    pub async fn new(lang: Lang) -> Self {
        let mut namespaces = HashMap::new();

        for ns in default_namespaces() {
            let json = load_http(lang, &ns).await;
            namespaces.insert(ns, json);
        }

        Self { namespaces }
    }
}

impl MessageLookup for MessagesHttp {
    fn namespaces(&self) -> &HashMap<Namespace, Value> {
        &self.namespaces
    }
}
