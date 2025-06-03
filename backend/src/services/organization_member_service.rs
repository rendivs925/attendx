use std::sync::Arc;

use crate::repositories::organization_member_repository::OrganizationMemberRepository;

use shared::models::organization_member_model::OrganizationMember;
use shared::types::requests::organization_member::register_organization_member_request::RegisterOrganizationMemberRequest;
use shared::types::requests::organization_member::update_organization_member_request::UpdateOrganizationMemberRequest;
use shared::types::responses::organization_member_response::OrganizationMemberResponse;
use shared::utils::locale_utils::{Messages, Namespace};

use mongodb::bson::to_document;

#[derive(Debug)]
pub enum OrganizationMemberServiceError {
    NotFound,
    DuplicateMember,
    DbError(String),
}

impl OrganizationMemberServiceError {
    pub fn to_message(&self, messages: &Messages) -> String {
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
        }
    }
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

    pub async fn create_member(
        &self,
        organization_id: &str,
        request: RegisterOrganizationMemberRequest,
    ) -> Result<OrganizationMemberResponse, OrganizationMemberServiceError> {
        let existing_member = self
            .organization_member_repository
            .find_member_by_org_and_name(organization_id, &request.name)
            .await
            .map_err(|e| OrganizationMemberServiceError::DbError(e.to_string()))?;

        if existing_member.is_some() {
            return Err(OrganizationMemberServiceError::DuplicateMember);
        }

        let member = OrganizationMember {
            organization_id: organization_id.to_string(),
            name: request.name,
            role: request.role,
            identifiers: request.identifiers,
            created_at: chrono::Utc::now(),
        };

        let created_member = self
            .organization_member_repository
            .create_member(&member)
            .await
            .map_err(|e| OrganizationMemberServiceError::DbError(e.to_string()))?;

        Ok(OrganizationMemberResponse::from(created_member))
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
        self.get_member_by_id(organization_id, name)
            .await?
            .ok_or(OrganizationMemberServiceError::NotFound)?;

        let update_doc = to_document(&updated).map_err(|e| {
            OrganizationMemberServiceError::DbError(format!(
                "Failed to convert update request to document: {}",
                e
            ))
        })?;

        let updated_member = self
            .organization_member_repository
            .update_member_by_org_and_name(organization_id, name, update_doc)
            .await
            .map_err(|e| OrganizationMemberServiceError::DbError(e.to_string()))?
            .ok_or(OrganizationMemberServiceError::NotFound)?;

        Ok(OrganizationMemberResponse::from(updated_member))
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
