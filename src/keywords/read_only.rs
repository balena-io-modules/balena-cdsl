use serde_json::Value;

use super::{CompilationResult, Compiler, WalkContextExt};
use crate::{Scope, WalkContext};

pub const KEYWORD: &str = "readOnly";

pub struct ReadOnly;

impl Compiler for ReadOnly {
    fn compile(&self, schema: &Value, ctx: &WalkContext, _scope: &Scope) -> CompilationResult {
        let value = compiler_non_strict_get!(schema, KEYWORD);

        if value.is_boolean() {
            Ok(None)
        } else {
            ctx.compilation_error(KEYWORD, "expected boolean")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn missing_keyword_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(ReadOnly.compile(&json!({}), &ctx, &scope).is_ok());
    }

    #[test]
    fn boolean_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(ReadOnly.compile(&json!({KEYWORD: false}), &ctx, &scope).is_ok());
        assert!(ReadOnly.compile(&json!({KEYWORD: true}), &ctx, &scope).is_ok());
    }

    #[test]
    fn other_types_not_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(ReadOnly.compile(&json!({KEYWORD: 10}), &ctx, &scope).is_err());
        assert!(ReadOnly.compile(&json!({KEYWORD: 10.2}), &ctx, &scope).is_err());
        assert!(ReadOnly.compile(&json!({KEYWORD: "foo"}), &ctx, &scope).is_err());
        assert!(ReadOnly.compile(&json!({KEYWORD: {}}), &ctx, &scope).is_err());
        assert!(ReadOnly.compile(&json!({KEYWORD: []}), &ctx, &scope).is_err());
    }
}