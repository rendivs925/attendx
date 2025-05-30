use log::warn;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::Path;

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
            "en" => Self::En,
            _ => Self::En,
        }
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lang::En => write!(f, "en"),
            Lang::Id => write!(f, "id"),
            Lang::De => write!(f, "de"),
            Lang::Ja => write!(f, "ja"),
        }
    }
}

fn load_message_file(lang: Lang, namespace: &str) -> Value {
    let lang_folder = lang.to_string();

    let file_path = Path::new("./shared/locales")
        .join(lang_folder)
        .join(format!("{namespace}.json"));

    match fs::read_to_string(&file_path) {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(json) => json,
            Err(err) => {
                eprintln!("[ERROR] Failed to parse JSON from {:?}: {}", file_path, err);
                Value::Null
            }
        },
        Err(err) => {
            eprintln!("[ERROR] Failed to read file {:?}: {}", file_path, err);
            Value::Null
        }
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
        match self {
            Namespace::Validation => write!(f, "validation"),
            Namespace::User => write!(f, "user"),
            Namespace::Auth => write!(f, "auth"),
            Namespace::Common => write!(f, "common"),
            Namespace::Organization => write!(f, "organization"),
            Namespace::Attendance => write!(f, "attendance"),
        }
    }
}

#[derive(Debug)]
pub struct Messages {
    pub namespaces: HashMap<Namespace, Value>,
}

#[derive(Debug)]
pub enum MessageError {
    MissingNamespace { namespace: Namespace },
    MissingKey { namespace: Namespace, path: String },
    InvalidType { namespace: Namespace, path: String },
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageError::MissingNamespace { namespace } => {
                write!(
                    f,
                    "Messages for namespace '{:?}' not loaded or found",
                    namespace
                )
            }
            MessageError::MissingKey { namespace, path } => {
                write!(f, "Missing message in {:?} at path '{}'", namespace, path)
            }
            MessageError::InvalidType { namespace, path } => {
                write!(
                    f,
                    "Expected string at {:?} path '{}', but found different type",
                    namespace, path
                )
            }
        }
    }
}

impl std::error::Error for MessageError {}

impl Messages {
    pub fn new(lang: Lang) -> Self {
        let mut namespaces = HashMap::new();

        let namespaces_to_load = [
            Namespace::User,
            Namespace::Validation,
            Namespace::Auth,
            Namespace::Common,
            Namespace::Organization,
            Namespace::Attendance,
        ];

        for &ns in &namespaces_to_load {
            let json_value = load_message_file(lang, &ns.to_string());
            assert!(
                json_value.is_object(),
                "Missing or invalid '{}' messages for language '{}'",
                ns.to_string(),
                lang.to_string()
            );
            namespaces.insert(ns, json_value);
        }

        Self { namespaces }
    }

    pub fn get(&self, namespace: &Namespace, path: &str) -> Option<&Value> {
        let root = self.namespaces.get(namespace)?;

        let mut current = root;
        for key in path.split('.') {
            current = current.get(key)?;
        }
        Some(current)
    }

    pub fn get_str(&self, namespace: Namespace, path: &str) -> Result<String, MessageError> {
        let value = self
            .get(&namespace, path)
            .ok_or_else(|| MessageError::MissingKey {
                namespace,
                path: path.to_string(),
            })?;

        value
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| MessageError::InvalidType {
                namespace,
                path: path.to_string(),
            })
    }

    pub fn get_message(&self, namespace: Namespace, key: &str) -> String {
        match self.get_str(namespace, key) {
            Ok(msg) => msg,
            Err(e) => {
                warn!(
                    "Failed to get message for namespace '{}' and key '{}': {}",
                    namespace, key, e
                );
                format!(
                    "Error: missing message for namespace '{}' and key '{}'",
                    namespace, key
                )
            }
        }
    }
}
