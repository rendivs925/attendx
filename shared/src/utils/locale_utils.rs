use log::warn;
use serde_json::Value;
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

fn load_message_file(lang: Lang, namespace: &str) -> Value {
    let lang_folder = match lang {
        Lang::En => "en",
        Lang::De => "de",
        Lang::Id => "id",
        Lang::Ja => "ja",
    };

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

#[derive(Debug, Clone)]
pub enum Namespace {
    Validation,
    User,
    Auth,
}

#[derive(Debug)]
pub struct Messages {
    pub user: Value,
    pub validation: Value,
    pub auth: Value,
}

#[derive(Debug)]
pub enum MessageError {
    MissingKey { namespace: Namespace, path: String },
    InvalidType { namespace: Namespace, path: String },
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
        let user = load_message_file(lang, "user");
        let validation = load_message_file(lang, "validation");
        let auth = load_message_file(lang, "auth");

        assert!(user.is_object(), "Missing or invalid 'user' messages");
        assert!(
            validation.is_object(),
            "Missing or invalid 'validation' messages"
        );
        assert!(auth.is_object(), "Missing or invalid 'auth' messages");

        Self {
            user,
            validation,
            auth,
        }
    }

    pub fn get(&self, namespace: &Namespace, path: &str) -> Option<&Value> {
        let root = match namespace {
            Namespace::User => &self.user,
            Namespace::Validation => &self.validation,
            Namespace::Auth => &self.auth,
        };

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
                namespace: namespace.clone(),
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

    pub fn get_user_message(&self, key: &str) -> String {
        match self.get_str(Namespace::User, key) {
            Ok(msg) => msg,
            Err(e) => {
                warn!("Failed to get user message for key '{}': {}", key, e);
                format!("Error: missing user message for key '{}'", key)
            }
        }
    }

    pub fn get_auth_message(&self, key: &str) -> String {
        match self.get_str(Namespace::Auth, key) {
            Ok(msg) => msg,
            Err(e) => {
                warn!("Failed to get auth message for key '{}': {}", key, e);
                format!("Error: missing auth message for key '{}'", key)
            }
        }
    }

    pub fn get_validation_message(&self, key: &str) -> String {
        match self.get_str(Namespace::Validation, key) {
            Ok(msg) => msg,
            Err(e) => {
                warn!("Failed to get validation message for key '{}': {}", key, e);
                format!("Error: missing validation message for key '{}'", key)
            }
        }
    }
}
