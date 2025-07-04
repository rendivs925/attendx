use crate::constants::API_BASE_URL;
use gloo_net::http::{Request, Response};
use leptos::web_sys::RequestCredentials;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::json;
use shared::types::responses::user_response::UserResponse;

const SUPABASE_URL: &str = "https://hxyuphznpsjkixxdmkan.supabase.co";
const SUPABASE_ANON_KEY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Imh4eXVwaHpucHNqa2l4eGRta2FuIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NTExMDk5NDIsImV4cCI6MjA2NjY4NTk0Mn0.CWRoNRxXqWezCko1AeUOZrGRfzzDcdMW_QXr0vbk-DY";

#[derive(Serialize, Clone)]
pub struct RegisterPayload {
    pub name: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Serialize, Clone)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub status_code: u16,
}

#[derive(Deserialize, Debug)]
pub struct SupabaseAuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub user: serde_json::Value,
}

#[derive(Deserialize, Debug)]
pub struct GraphQLData {
    pub register_user: UserResponse,
}

#[derive(Deserialize, Debug)]
pub struct GraphQLLoginData {
    pub login_user: UserResponse,
}

#[derive(Deserialize, Debug)]
pub struct GraphQLResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Deserialize, Debug)]
pub struct GraphQLError {
    pub message: String,
}

fn extract_graphql_error(errors: Option<Vec<GraphQLError>>) -> Option<String> {
    errors.map(|errs| {
        errs.into_iter()
            .map(|e| e.message)
            .collect::<Vec<_>>()
            .join("; ")
    })
}

pub async fn send_register_request(payload: &RegisterPayload) -> Result<UserResponse, String> {
    if payload.password != payload.password_confirmation {
        return Err("Password confirmation does not match".into());
    }

    debug!("Registering user: {}", payload.email);

    let auth_res = Request::post(&format!("{SUPABASE_URL}/auth/v1/signup"))
        .header("Content-Type", "application/json")
        .header("apikey", SUPABASE_ANON_KEY)
        .body(
            json!({
                "email": payload.email,
                "password": payload.password,
                "data": { "name": payload.name }
            })
            .to_string(),
        )
        .map_err(|e| format!("Failed to build auth request body: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Supabase signup error: {e}"))?;

    if !auth_res.ok() {
        let body = auth_res.text().await.unwrap_or_default();
        return Err(format!("Supabase error: {} ({})", auth_res.status(), body));
    }

    let auth_json: SupabaseAuthResponse = auth_res
        .json()
        .await
        .map_err(|e| format!("Auth parse error: {e}"))?;

    if auth_json.access_token.is_empty() {
        return Err("Supabase returned empty access token".into());
    }

    let gql_res = Request::post(&format!("{}/graphql", *API_BASE_URL))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", auth_json.access_token))
        .credentials(RequestCredentials::Include)
        .body(
            json!({
                "query": "mutation Register($input: RegisterRequest!) { registerUser(input: $input) { id name email } }",
                "variables": {
                    "input": {
                        "name": payload.name,
                        "email": payload.email
                    }
                }
            })
            .to_string(),
        )
        .map_err(|e| format!("Failed to build GraphQL request body: {e}"))?
        .send()
        .await
        .map_err(|e| format!("GraphQL error: {e}"))?;

    if !gql_res.ok() {
        let body = gql_res.text().await.unwrap_or_default();
        return Err(format!(
            "GraphQL returned status {}: {}",
            gql_res.status(),
            body
        ));
    }

    let gql_json: GraphQLResponse<GraphQLData> = gql_res
        .json()
        .await
        .map_err(|e| format!("GraphQL parse error: {e}"))?;

    if let Some(message) = extract_graphql_error(gql_json.errors) {
        return Err(format!("GraphQL error: {message}"));
    }

    gql_json
        .data
        .map(|d| d.register_user)
        .ok_or_else(|| "Missing GraphQL data".into())
}

pub async fn send_login_request(payload: &LoginPayload) -> Result<UserResponse, String> {
    let res = Request::post(&format!("{SUPABASE_URL}/auth/v1/token?grant_type=password"))
        .header("Content-Type", "application/json")
        .header("apikey", SUPABASE_ANON_KEY)
        .header("Authorization", &format!("Bearer {SUPABASE_ANON_KEY}"))
        .body(
            json!({
                "email": payload.email,
                "password": payload.password
            })
            .to_string(),
        )
        .map_err(|e| format!("Failed to build login request body: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Supabase login error: {e}"))?;

    if !res.ok() {
        let body = res.text().await.unwrap_or_default();
        return Err(format!(
            "Supabase login failed: {} ({})",
            res.status(),
            body
        ));
    }

    let auth_json: SupabaseAuthResponse = res
        .json()
        .await
        .map_err(|e| format!("Auth parse error: {e}"))?;

    if auth_json.access_token.is_empty() {
        return Err("Supabase returned empty access token".into());
    }

    let gql_res = Request::post(&format!("{}/graphql", *API_BASE_URL))
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            &format!("Bearer {}", auth_json.access_token),
        )
        .credentials(RequestCredentials::Include)
        .body(
            json!({
                "query": "query Me { loginUser { id name email } }"
            })
            .to_string(),
        )
        .map_err(|e| format!("Failed to build GraphQL login request body: {e}"))?
        .send()
        .await
        .map_err(|e| format!("GraphQL fetch error: {e}"))?;

    if !gql_res.ok() {
        let body = gql_res.text().await.unwrap_or_default();
        return Err(format!(
            "GraphQL returned status {}: {}",
            gql_res.status(),
            body
        ));
    }

    let gql_json: GraphQLResponse<GraphQLLoginData> = gql_res
        .json()
        .await
        .map_err(|e| format!("GraphQL parse error: {e}"))?;

    if let Some(message) = extract_graphql_error(gql_json.errors) {
        return Err(format!("GraphQL error: {message}"));
    }

    gql_json
        .data
        .map(|d| d.login_user)
        .ok_or_else(|| "Missing GraphQL login data".into())
}
