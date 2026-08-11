use serde_json::json;

use crate::SchemaBundle;

pub(crate) fn schemas() -> SchemaBundle {
    let root = json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "$id": "https://norm-spec.dev/schema/norm-schema.json",
        "type": "object",
        "additionalProperties": false,
        "required": ["metadata"],
        "properties": {
            "metadata": {
                "type": "object",
                "required": ["layer", "scope", "version"],
                "properties": {
                    "layer": {"type": "string"},
                    "profile": {"type": "string"},
                    "scope": {"type": "string"},
                    "version": {"type": "string", "pattern": "^\\d+\\.\\d+$"}
                }
            },
            "template": {"type": "object"},
            "agent_rules": {"type": "object"},
            "cross_references": {"type": "array"},
            "strong_references": {"type": "array"},
            "scope": {"type": "object"}
        }
    });
    let convention = json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "$id": "https://norm-spec.dev/schema/profiles/convention.json",
        "allOf": [{"$ref": "../norm-schema.json"}],
        "profile_rules": {
            "required_fields": ["metadata"],
            "recommended_fields": ["template"],
            "recommended_template": ["required_files"]
        }
    });
    SchemaBundle::new(root, [("convention".to_owned(), convention)])
}
