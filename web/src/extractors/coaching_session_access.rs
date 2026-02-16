use axum::{async_trait, extract::{FromRef, FromRequestParts, Path}, http::{StatusCode, request::Parts}};
use domain::{Id, coaching_session};

use crate::{AppState, extractors::{RejectionType, authenticated_user::AuthenticatedUser}};
use domain::users;
use domain::coaching_sessions;
use domain::coaching_relationship;
// use sea_orm::{DatabaseConnection};
use log::*;

pub(crate) struct CoachingSessionAccess {
    pub coaching_session: coaching_sessions::Model,
    #[allow(dead_code)]
    pub authenticated_user: users::Model,
}

// #[async_trait]
// impl<S> FromRequestParts<S> for CoachingSessionAccess
// where
//     S: Send + Sync,
//     DatabaseConnection: FromRequestParts<S>,
// {
//     type Rejection = RejectionType;

//     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         // let mut db = DatabaseConnection::from_request_parts(parts, state).await?;
//         let db = match DatabaseConnection::from_request_parts(parts, state)
//             .await
//             // .map_err(|e| AuthSessionError::DatabaseError(e.to_string()))?;
//             // .map_err(|_e| (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))?;
//             {
//                 Ok(connection) => connection,
//                 Err(_e) => {
//                     // error!("Error finding coaching relationship: {e:?}");
//                     return Err((StatusCode::INTERNAL_SERVER_ERROR, "Please try again later".to_string()));
//                 }
//             };

//         let AuthenticatedUser(authenticated_user) = AuthenticatedUser::from_request_parts(parts, state).await?;

//         let Path(coaching_session_id) = match Path::<Id>::from_request_parts(parts, state)
//             .await
//             {
//                 Ok(path) => path,
//                 Err(_e) => {
//                     return Err((StatusCode::BAD_REQUEST, "Invalid coaching session id".to_string()));
//                 }
//             };
//         debug!("GET Coaching Session by ID: {coaching_session_id}");

//         // let session: domain::user::AuthSession = AuthSession::from_request_parts(parts, state)
//         //     .await
//         //     .map_err(|(status, msg)| (status, msg.to_string()))?;

//         // match session.user {
//         //     Some(user) => Ok(AuthenticatedUser(user)),
//         //     None => Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string())),
//         // }

//         // Get the coaching session
//         let coaching_session = match coaching_session::find_by_id(
//             &db,
//             coaching_session_id,
//         )
//         .await
//         {
//             Ok(session) => session,
//             Err(e) => {
//                 error!("Error finding coaching session {coaching_session_id}: {e:?}");
//                 return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
//             }
//         };

//         debug!("Found Coaching Session: {coaching_session:?}");

//         // Get the coaching relationship
//         let coaching_relationship = match coaching_relationship::find_by_id(
//             &db,
//             coaching_session.coaching_relationship_id,
//         )
//         .await
//         {
//             Ok(relationship) => relationship,
//             Err(e) => {
//                 error!(
//                     "Error finding coaching relationship {}: {e:?}",
//                     coaching_session.coaching_relationship_id
//                 );
//                 return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
//             }
//         };

//         // Check if user is coach or coachee
//         if (
//             coaching_relationship.coach_id == authenticated_user.id
//             || coaching_relationship.coachee_id == authenticated_user.id) == false
//         {
//             return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
//         }

//         Ok(
//             CoachingSessionAccess{
//                 coaching_session,
//                 authenticated_user
//             }
//         )
//     }
// }

#[async_trait]
impl<S> FromRequestParts<S> for CoachingSessionAccess
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = RejectionType;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        let AuthenticatedUser(authenticated_user) = AuthenticatedUser::from_request_parts(parts, &state).await?;

        let Path(coaching_session_id) = match Path::<Id>::from_request_parts(parts, &state)
            .await
            {
                Ok(path) => path,
                Err(_e) => {
                    return Err((StatusCode::BAD_REQUEST, "Invalid coaching session id".to_string()));
                }
            };
        debug!("GET Coaching Session by ID: {coaching_session_id}");

        // let session: domain::user::AuthSession = AuthSession::from_request_parts(parts, state)
        //     .await
        //     .map_err(|(status, msg)| (status, msg.to_string()))?;

        // match session.user {
        //     Some(user) => Ok(AuthenticatedUser(user)),
        //     None => Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string())),
        // }

        // Get the coaching session
        let coaching_session = match coaching_session::find_by_id(
            state.db_conn_ref(),
            coaching_session_id,
        )
        .await
        {
            Ok(session) => session,
            Err(e) => {
                error!("Error finding coaching session {coaching_session_id}: {e:?}");
                return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
            }
        };

        debug!("Found Coaching Session: {coaching_session:?}");

        // Get the coaching relationship
        let coaching_relationship = match coaching_relationship::find_by_id(
            state.db_conn_ref(),
            coaching_session.coaching_relationship_id,
        )
        .await
        {
            Ok(relationship) => relationship,
            Err(e) => {
                error!(
                    "Error finding coaching relationship {}: {e:?}",
                    coaching_session.coaching_relationship_id
                );
                return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
            }
        };

        // Check if user is coach or coachee
        if (
            coaching_relationship.coach_id == authenticated_user.id
            || coaching_relationship.coachee_id == authenticated_user.id) == false
        {
            return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
        }

        Ok(
            CoachingSessionAccess { coaching_session, authenticated_user }
        )
    }
}

// #[cfg(test)]
// #[cfg(feature = "mock")]
// mod tests {
//     use super::*;
//     use axum::http::request::Parts;
//     use axum::extract::{FromRequestParts, Path};
//     use sea_orm::{DatabaseBackend, MockDatabase};

//     // Mock AuthenticatedUser extractor for testing
//     struct MockAuthenticatedUserExtractor;

//     impl<S> FromRequestParts<S> for AuthenticatedUser
//     where
//         S: Send + Sync,
//     {
//         type Rejection = (StatusCode, String);

//         async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//             // In tests, we'll set the user in extensions manually
//             parts.extensions
//                 .get::<AuthenticatedUser>()
//                 .cloned()
//                 .ok_or((StatusCode::UNAUTHORIZED, "No user".to_string()))
//         }
//     }

//     #[tokio::test]
//     async fn test_extractor_success() {
//         // Create mock database with expected results
//         let db = MockDatabase::new(DatabaseBackend::Postgres)
//             .append_query_results(vec![
//                 vec![coaching_session_model(1, 100)],
//             ])
//             .append_query_results(vec![
//                 vec![coaching_relationship_model(100, 1, 2)],
//             ])
//             .into_connection();

//         // Create app state
//         let state = AppState {
//             db_conn: db,
//             // ... other fields
//         };

//         // Create request parts with path and user
//         let mut parts = Parts::default();

//         // Add path parameters
//         parts.uri = "/coaching_sessions/1".parse().unwrap();

//         // Add authenticated user
//         parts.extensions.insert(AuthenticatedUser(
//             users::Model {
//                 id: 1,
//                 // ... other fields
//             }
//         ));

//         // Test the extractor
//         let result = CoachingSessionAccess::from_request_parts(&mut parts, &state).await;

//         assert!(result.is_ok());
//         let access = result.unwrap();
//         assert_eq!(access.coaching_session.id, 1);
//         assert_eq!(access.authenticated_user.id, 1);
//     }
// }
