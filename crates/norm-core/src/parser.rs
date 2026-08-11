//! A1 and explicitly enabled pre-A1 parsing.

use std::{error::Error, fmt};

use serde_json::Value;

/// Options that alter accepted `.norm` input formats.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ParseOptions {
    /// Accept the pre-A1 `# Title` followed by YAML compatibility format.
    pub legacy_format: bool,
}

/// A parsed `.norm` document before it is wrapped in a machine response.
#[derive(Clone, Debug, PartialEq)]
pub struct ParsedNorm {
    /// YAML frontmatter represented as JSON-compatible data.
    pub frontmatter: Value,
    /// Trimmed Markdown body.
    pub body: String,
}

/// Stable parser failure categories.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// The document or its frontmatter is empty.
    Empty,
    /// An A1 opening fence has no closing fence.
    MissingClosingFence,
    /// The document is not A1 and compatibility parsing was not enabled.
    NotA1,
    /// The YAML document root is not a mapping.
    RootType,
    /// YAML deserialization failed.
    Yaml(String),
}

impl ParseError {
    /// Return the stable machine-readable error code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Empty => "norm/parse/empty",
            Self::MissingClosingFence => "norm/parse/missing-closing-fence",
            Self::NotA1 => "norm/parse/not-a1",
            Self::RootType => "norm/parse/root-type",
            Self::Yaml(_) => "norm/parse/yaml",
        }
    }

    /// Return a stable human-readable diagnostic.
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::Empty => "The .norm document or its frontmatter is empty.".to_owned(),
            Self::MissingClosingFence => {
                "A1 frontmatter opened without a closing fence.".to_owned()
            }
            Self::NotA1 => {
                "The document is not A1; use --legacy-format for pre-A1 input.".to_owned()
            }
            Self::RootType => "The YAML frontmatter root must be a mapping.".to_owned(),
            Self::Yaml(detail) => format!("The YAML frontmatter is invalid: {detail}"),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message())
    }
}

impl Error for ParseError {}

/// Parse an A1 `.norm` document, or the pre-A1 format when explicitly enabled.
///
/// # Errors
///
/// Returns a stable [`ParseError`] category for empty input, malformed fences,
/// unsupported input format, a non-mapping root, or invalid YAML.
pub fn parse_norm(input: &str, options: ParseOptions) -> Result<ParsedNorm, ParseError> {
    let input = input.strip_prefix('\u{feff}').unwrap_or(input);
    if input.trim().is_empty() {
        return Err(ParseError::Empty);
    }

    let lines: Vec<&str> = input.lines().collect();
    let Some(first_content) = lines.iter().position(|line| !line.trim().is_empty()) else {
        return Err(ParseError::Empty);
    };

    if lines[first_content].trim() == "---" {
        parse_a1(&lines, first_content)
    } else if options.legacy_format {
        parse_legacy(&lines)
    } else {
        Err(ParseError::NotA1)
    }
}

fn parse_a1(lines: &[&str], opening: usize) -> Result<ParsedNorm, ParseError> {
    let Some(closing) = lines
        .iter()
        .enumerate()
        .skip(opening + 1)
        .find_map(|(index, line)| (line.trim() == "---").then_some(index))
    else {
        return Err(ParseError::MissingClosingFence);
    };

    let yaml = lines[opening + 1..closing].join("\n");
    let frontmatter = parse_frontmatter(&yaml)?;
    let body = lines[closing + 1..].join("\n").trim().to_owned();

    Ok(ParsedNorm { frontmatter, body })
}

fn parse_legacy(lines: &[&str]) -> Result<ParsedNorm, ParseError> {
    let yaml_start = lines
        .iter()
        .position(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !trimmed.starts_with('#')
        })
        .ok_or(ParseError::Empty)?;

    let body = lines[..yaml_start].join("\n").trim().to_owned();
    let yaml = lines[yaml_start..].join("\n");
    let frontmatter = parse_frontmatter(&yaml)?;

    Ok(ParsedNorm { frontmatter, body })
}

fn parse_frontmatter(yaml: &str) -> Result<Value, ParseError> {
    if yaml.trim().is_empty() {
        return Err(ParseError::Empty);
    }

    let value: Value =
        serde_saphyr::from_str(yaml).map_err(|error| ParseError::Yaml(error.to_string()))?;
    if !value.is_object() {
        return Err(ParseError::RootType);
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{ParseError, ParseOptions, parse_norm};

    const A1: &str = "---\nmetadata:\n  version: \"1.0\"\n---\n\n# Body\n";

    #[test]
    fn parses_a1_frontmatter_and_trimmed_body() {
        let parsed = parse_norm(A1, ParseOptions::default());
        assert_eq!(
            parsed,
            Ok(super::ParsedNorm {
                frontmatter: json!({"metadata": {"version": "1.0"}}),
                body: "# Body".to_owned(),
            })
        );
    }

    #[test]
    fn accepts_bom_and_leading_blank_lines() {
        let parsed = parse_norm(&format!("\u{feff}\n\n{A1}"), ParseOptions::default());
        assert!(parsed.is_ok());
    }

    #[test]
    fn legacy_mode_preserves_leading_markdown_comments() {
        let parsed = parse_norm(
            "# Legacy title\n\nmetadata:\n  version: \"1.0\"\n",
            ParseOptions {
                legacy_format: true,
            },
        );
        assert_eq!(
            parsed,
            Ok(super::ParsedNorm {
                frontmatter: json!({"metadata": {"version": "1.0"}}),
                body: "# Legacy title".to_owned(),
            })
        );
    }

    #[test]
    fn rejects_legacy_input_without_opt_in() {
        let error = parse_norm("metadata: {}", ParseOptions::default());
        assert_eq!(error, Err(ParseError::NotA1));
    }

    #[test]
    fn classifies_empty_documents_and_frontmatter() {
        assert_eq!(
            parse_norm("\n\t", ParseOptions::default()),
            Err(ParseError::Empty)
        );
        assert_eq!(
            parse_norm("---\n---", ParseOptions::default()),
            Err(ParseError::Empty)
        );
    }

    #[test]
    fn classifies_missing_closing_fence() {
        assert_eq!(
            parse_norm("---\nmetadata: {}", ParseOptions::default()),
            Err(ParseError::MissingClosingFence)
        );
    }

    #[test]
    fn rejects_non_mapping_root() {
        assert_eq!(
            parse_norm("---\n- metadata\n---", ParseOptions::default()),
            Err(ParseError::RootType)
        );
    }

    #[test]
    fn classifies_invalid_yaml_without_panicking() {
        let result = parse_norm("---\nmetadata: [unterminated\n---", ParseOptions::default());
        assert!(matches!(result, Err(ParseError::Yaml(_))));
    }
}
