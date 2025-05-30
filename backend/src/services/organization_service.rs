use crate::{
    repositories::organization_repository::OrganizationRepository, utils::auth_utils::hash_password,
};
use log::error;
use shared::{
    models::organization_model::Organization,
    types::{
        requests::organization::register_organization_request::RegisterOrganizationRequest,
        responses::organization_response::OrganizationResponse,
    },
    utils::locale_utils::{Messages, Namespace},
};
use std::sync::Arc;

#[derive(Debug)]
pub enum OrganizationServiceError {
    NotFound,
    InvalidData,
    DuplicateEmail,
    DbError(String),
    JwtGenerationError(String),
    PasswordHashingError(String),
}

impl OrganizationServiceError {
    pub fn to_message(&self, messages: &Messages) -> String {
        match self {
            OrganizationServiceError::NotFound => {
                messages.get_message(Namespace::Organization, "fetch.not_found")
            }
            OrganizationServiceError::InvalidData => {
                messages.get_message(Namespace::Common, "invalid_data")
            }
            OrganizationServiceError::DuplicateEmail => {
                messages.get_message(Namespace::Organization, "create.duplicate_email")
            }
            OrganizationServiceError::DbError(_) => {
                messages.get_message(Namespace::Common, "db_error")
            }
            OrganizationServiceError::JwtGenerationError(_) => {
                messages.get_message(Namespace::Common, "jwt_generation_failed")
            }
            OrganizationServiceError::PasswordHashingError(_) => {
                messages.get_message(Namespace::Common, "password_hashing_failed")
            }
        }
    }
}

pub struct OrganizationService {
    organization_repository: Arc<OrganizationRepository>,
}

impl OrganizationService {
    pub fn new(organization_repository: Arc<OrganizationRepository>) -> Self {
        Self {
            organization_repository,
        }
    }

    pub async fn create_organization(
        &self,
        new_organization: RegisterOrganizationRequest,
    ) -> Result<OrganizationResponse, OrganizationServiceError> {
        let existing_org = self
            .organization_repository
            .find_organization("email", &new_organization.email)
            .await
            .map_err(|e| OrganizationServiceError::DbError(e.to_string()))?;

        if existing_org.is_some() {
            return Err(OrganizationServiceError::DuplicateEmail);
        }

        let hashed_password = hash_password(&new_organization.password).map_err(|e| {
            error!(
                "Failed to hash password for organization {}: {:?}",
                new_organization.email, e
            );
            OrganizationServiceError::PasswordHashingError(e.to_string())
        })?;

        let organization = Organization {
            name: new_organization.name,
            email: new_organization.email.clone(),
            password: hashed_password,
            logo_url: new_organization.logo_url,
            ..Default::default()
        };

        let created = self
            .organization_repository
            .create_organization(organization)
            .await
            .map_err(|e| {
                error!(
                    "Database error when creating organization {}: {:?}",
                    new_organization.email, e
                );
                OrganizationServiceError::DbError(e.to_string())
            })?;

        Ok(OrganizationResponse::from(created))
    }

    pub async fn get_organization_by_id(
        &self,
        org_id: &str,
    ) -> Result<Option<OrganizationResponse>, OrganizationServiceError> {
        let org = self
            .organization_repository
            .find_organization_by_id(org_id)
            .await
            .map_err(|e| OrganizationServiceError::DbError(e.to_string()))?;

        Ok(org.map(OrganizationResponse::from))
    }

    pub async fn get_all_organizations(
        &self,
    ) -> Result<Vec<OrganizationResponse>, OrganizationServiceError> {
        let orgs = self
            .organization_repository
            .get_all_organizations()
            .await
            .map_err(|e| OrganizationServiceError::DbError(e.to_string()))?;

        Ok(orgs.into_iter().map(OrganizationResponse::from).collect())
    }

    pub async fn update_organization(
        &self,
        org_id: &str,
        organization: Organization,
    ) -> Result<OrganizationResponse, OrganizationServiceError> {
        let updated = self
            .organization_repository
            .update_organization(org_id, &organization)
            .await
            .map_err(|e| OrganizationServiceError::DbError(e.to_string()))?;

        Ok(OrganizationResponse::from(updated))
    }

    pub async fn delete_organization(&self, org_id: &str) -> Result<(), OrganizationServiceError> {
        self.organization_repository
            .delete_organization(org_id)
            .await
            .map_err(|e| OrganizationServiceError::DbError(e.to_string()))
    }
}
