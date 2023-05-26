use async_graphql::{async_trait::async_trait, Context, Guard, Result};

use crate::{auth::Scope, error::new_err};

pub struct ScopeGuard {
    required_scope: String,
}

impl ScopeGuard {
    pub fn new<T: Into<String>>(required_scope: T) -> Self {
        Self {
            required_scope: required_scope.into(),
        }
    }
}

#[async_trait]
impl Guard for ScopeGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let scopes = ctx.data_unchecked::<Vec<Scope>>();

        match has_scopes(scopes, &self.required_scope) {
            Ok(true) => Ok(()),
            Ok(false) => Err(new_err(
                "UNAUTHORIZED",
                "You do not have the required scope permissions to perform this action",
            )),
            Err(e) => Err(e),
        }
    }
}

fn has_scopes(scopes: &Vec<Scope>, required_scope: &str) -> async_graphql::Result<bool> {
    let required_scope_parts: Vec<&str> = required_scope.split(':').collect();

    for scope in scopes {
        if scope.0 == "*" {
            return Ok(true);
        }
        let scope_parts: Vec<&str> = scope.0.split(':').collect();
        if scope_parts.len() == 0 {
            return Err(new_err("INVALID_SCOPE", "Scope with no parts was provided"));
        }
        if has_scope(&scope_parts, &required_scope_parts) {
            return Ok(true);
        }
    }

    return Ok(false);
}

fn has_scope(scope_parts: &[&str], required_scope_parts: &[&str]) -> bool {
    match (scope_parts.get(0), required_scope_parts.get(0)) {
        (Some(scope), Some(part)) => {
            if scope == part {
                // check deeper
                return has_scope(&scope_parts[1..], &required_scope_parts[1..]);
            } else {
                // if the scope part is not equal, then they are different scopes
                return false;
            }
        }

        // if the scope is None, but there is more required parts
        // then they have a more general scope
        (None, Some(part)) => return true,
        // if they have more scope parts, but no more required parts
        // then they have a more specific scope and we can return false
        (Some(_), None) => return false,
        // if they both have no more parts, then they are equal
        // and we can return true
        (None, None) => return true,
    }
}

#[cfg(test)]
mod tests {
    use crate::auth::Scope;

    #[test]
    fn works_for_exact_scope() {
        let user_scopes = vec![
            Scope("read:users".to_string()),
            Scope("write:users".to_string()),
        ];
        let required_scope = "read:users";
        assert_eq!(
            super::has_scopes(&user_scopes, required_scope).unwrap(),
            true
        );
    }

    #[test]
    fn succeeds_for_parent_scope() {
        let user_scopes = vec![Scope("profile".to_string())];
        let required_scope = "profile:read";

        assert_eq!(
            super::has_scopes(&user_scopes, required_scope).unwrap(),
            true
        );
    }

    #[test]
    fn fails_for_less_general_scope() {
        let user_scopes = vec![Scope("profile:read".to_string())];
        let required_scope = "profile";

        assert_eq!(
            super::has_scopes(&user_scopes, required_scope).unwrap(),
            false
        );
    }

    #[test]
    fn succeeds_for_more_general_scope() {
        let user_scopes = vec![Scope("profile".to_string())];
        let required_scope = "profile:read";

        assert_eq!(
            super::has_scopes(&user_scopes, required_scope).unwrap(),
            true
        );
    }

    #[test]
    fn succeeds_for_multiple_scopes() {
        let user_scopes = vec![
            Scope("profile:read".to_string()),
            Scope("profile:write".to_string()),
            Scope("profile:delete".to_string()),
        ];
        let required_scope = "profile:read";

        assert_eq!(
            super::has_scopes(&user_scopes, required_scope).unwrap(),
            true
        );
    }

    #[test]
    fn fails_for_missing_scope() {
        let user_scopes = vec![
            Scope("profile:read".to_string()),
            Scope("profile:write".to_string()),
            Scope("profile:delete".to_string()),
        ];
        let required_scope = "profile:manage";

        assert_eq!(
            super::has_scopes(&user_scopes, required_scope).unwrap(),
            false
        );
    }

    #[test]
    fn fails_for_invalid_scope() {
        let user_scopes = vec![Scope("::".to_string())];
        let required_scope = "profile:read:write";

        assert_eq!(
            super::has_scopes(&user_scopes, required_scope).unwrap(),
            false
        );
    }
}
