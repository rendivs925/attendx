use std::sync::Arc;

use crate::repositories::organization_member_repository::OrganizationMemberRepository;
use crate::utils::auth_utils::{generate_jwt, verify_password};

use shared::models::organization_member_model::OrganizationMember;
use shared::prelude::*;
use shared::types::requests::organization_member::register_organization_member_request::RegisterOrganizationMemberRequest;
use shared::types::requests::organization_member::update_organization_member_request::UpdateOrganizationMemberRequest;
use shared::types::responses::organization_member_response::OrganizationMemberResponse;
use shared::utils::locale_utils::Namespace;

use chrono::Utc;
use mongodb::bson::to_document;

#[derive(Debug)]
pub enum OrganizationMemberServiceError {
    NotFound,
    DuplicateMember,
    DbError(String),
    InvalidCredentials,
    JwtGenerationError(String),
}

impl OrganizationMemberServiceError {
    pub fn to_message(&self, messages: &dyn MessageLookup) -> String {
        match self {
            OrganizationMemberServiceError::NotFound => {
                messages.get_message(Namespace::OrganizationMember, "fetch.not_found")
            }
            OrganizationMemberServiceError::DuplicateMember => {
                messages.get_message(Namespace::OrganizationMember, "create.duplicate")
            }
            OrganizationMemberServiceError::DbError(_) => {
                messages.get_message(Namespace::OrganizationMember, "db_error")
            }
            OrganizationMemberServiceError::InvalidCredentials => {
                messages.get_message(Namespace::OrganizationMember, "auth.invalid_credentials")
            }
            OrganizationMemberServiceError::JwtGenerationError(_) => {
                messages.get_message(Namespace::OrganizationMember, "auth.jwt_generation_error")
            }
        }
    }
}

#[derive(Debug)]
pub enum OrganizationMemberAuthError {
    NotFound,
    InvalidCredentials,
    DbError(String),
    JwtGenerationError(String),
}

pub struct OrganizationMemberService {
    pub organization_member_repository: Arc<OrganizationMemberRepository>,
}

impl OrganizationMemberService {
    pub fn new(organization_member_repository: Arc<OrganizationMemberRepository>) -> Self {
        Self {
            organization_member_repository,
        }
    }

    pub async fn authenticate_member(
        &self,
        organization_id: &str,
        name: &str,
        password: &str,
    ) -> Result<(OrganizationMemberResponse, String), OrganizationMemberAuthError> {
        let member = self
            .organization_member_repository
            .find_member_by_org_and_name(organization_id, name)
            .await
            .map_err(|e| OrganizationMemberAuthError::DbError(e.to_string()))?
            .ok_or(OrganizationMemberAuthError::NotFound)?;

        let stored_password = member
            .identifiers
            .get("password")
            .ok_or(OrganizationMemberAuthError::InvalidCredentials)?;

        if !verify_password(password, stored_password) {
            return Err(OrganizationMemberAuthError::InvalidCredentials);
        }

        let email = member
            .identifiers
            .get("email")
            .ok_or(OrganizationMemberAuthError::InvalidCredentials)?;

        let token = generate_jwt(&member.name, email)
            .map_err(OrganizationMemberAuthError::JwtGenerationError)?;

        Ok((OrganizationMemberResponse::from(member), token))
    }

    pub async fn create_member(
        &self,
        organization_id: &str,
        request: RegisterOrganizationMemberRequest,
    ) -> Result<OrganizationMemberResponse, OrganizationMemberServiceError> {
        let exists = self
            .organization_member_repository
            .find_member_by_org_and_name(organization_id, &request.name)
            .await
            .map_err(|e| OrganizationMemberServiceError::DbError(e.to_string()))?;

        if exists.is_some() {
            return Err(OrganizationMemberServiceError::DuplicateMember);
        }

        let member = OrganizationMember {
            organization_id: organization_id.to_string(),
            name: request.name,
            role: request.role,
            identifiers: request.identifiers,
            created_at: Utc::now(),
        };

        let created = self
            .organization_member_repository
            .create_member(&member)
            .await
            .map_err(|e| OrganizationMemberServiceError::DbError(e.to_string()))?;

        Ok(OrganizationMemberResponse::from(created))
    }

    pub async fn get_member_by_id(
        &self,
        organization_id: &str,
        name: &str,
    ) -> Result<Option<OrganizationMemberResponse>, OrganizationMemberServiceError> {
        let member = self
            .organization_member_repository
            .find_member_by_org_and_name(organization_id, name)
            .await
            .map_err(|e| OrganizationMemberServiceError::DbError(e.to_string()))?;

        Ok(member.map(OrganizationMemberResponse::from))
    }

    pub async fn get_all_members(
        &self,
        organization_id: &str,
    ) -> Result<Vec<OrganizationMemberResponse>, OrganizationMemberServiceError> {
        let members = self
            .organization_member_repository
            .get_all_members_by_org(organization_id)
            .await
            .map_err(|e| OrganizationMemberServiceError::DbError(e.to_string()))?;

        Ok(members
            .into_iter()
            .map(OrganizationMemberResponse::from)
            .collect())
    }

    pub async fn update_member(
        &self,
        organization_id: &str,
        name: &str,
        updated: UpdateOrganizationMemberRequest,
    ) -> Result<OrganizationMemberResponse, OrganizationMemberServiceError> {
        let _existing = self
            .get_member_by_id(organization_id, name)
            .await?
            .ok_or(OrganizationMemberServiceError::NotFound)?;

        let update_doc = to_document(&updated).map_err(|e| {
            OrganizationMemberServiceError::DbError(format!(
                "Failed to convert update request to document: {}",
                e
            ))
        })?;

        let updated = self
            .organization_member_repository
            .update_member_by_org_and_name(organization_id, name, update_doc)
            .await
            .map_err(|e| OrganizationMemberServiceError::DbError(e.to_string()))?
            .ok_or(OrganizationMemberServiceError::NotFound)?;

        Ok(OrganizationMemberResponse::from(updated))
    }

    pub async fn delete_member(
        &self,
        organization_id: &str,
        name: &str,
    ) -> Result<(), OrganizationMemberServiceError> {
        self.organization_member_repository
            .delete_member_by_org_and_name(organization_id, name)
            .await
            .map_err(|e| OrganizationMemberServiceError::DbError(e.to_string()))?;

        Ok(())
    }
}
