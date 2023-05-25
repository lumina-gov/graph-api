use async_graphql::{async_trait::async_trait, Context, Guard, Result};

use crate::{error::new_err, graphql::types::user::User};

pub struct ScopeGuard;

#[async_trait]
impl Guard for ScopeGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        unimplemented!()
    }
}

fn has_scopes(scopes: &Vec<String>, required_scope: &str) -> bool {
    let required_scope_parts: Vec<&str> = required_scope.split(':').collect();

    for scope in scopes {
        if has_scope(scope, &required_scope_parts) {
            return true;
        }
    }

    return false;
}

fn has_scope(scope: &str, required_scope: &[&str]) -> bool {
    unimplemented!()
}

#[cfg(test)]
mod tests {

    #[test]
    fn works_for_exact_scope() {
        let user_scopes = vec!["read:users".to_string(), "write:users".to_string()];
        let required_scope = "read:users";
        assert_eq!(super::has_scopes(&user_scopes, required_scope), true);
    }

    #[test]
    fn fails_for_missing_scope() {
        let user_scopes = vec!["profile:read".to_string(), "profile".to_string()];
        let required_scope = "applications:create";

        assert_eq!(super::has_scopes(&user_scopes, required_scope), false);
    }

    #[test]
    fn succeeds_for_parent_scope() {
        let user_scopes = vec!["profile".to_string()];
        let required_scope = "profile:read";

        assert_eq!(super::has_scopes(&user_scopes, required_scope), true);
    }
}
