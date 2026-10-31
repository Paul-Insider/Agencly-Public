//! A small, dependency-free example of validating residence branding in Rust.
//!
//! This module demonstrates a typed boundary between raw strings and validated
//! presentation data. It is independent of the commercial application. A public
//! identifier is a display identifier, not authentication or tenant authority.

use std::fmt;

/// Limits belong to this example's contract, not the commercial product schema.
pub const MAX_IDENTIFIER_BYTES: usize = 48;
pub const MAX_DISPLAY_CHARACTERS: usize = 80;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResidenceBrand {
    identifier: String,
    name: String,
    destination: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrandingError {
    InvalidIdentifier,
    InvalidName,
    InvalidDestination,
}

impl fmt::Display for BrandingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidIdentifier => "Use a lowercase identifier with single internal hyphens",
            Self::InvalidName => "Provide a non-empty, bounded residence name without controls",
            Self::InvalidDestination => "Provide a non-empty, bounded destination without controls",
        })
    }
}

impl std::error::Error for BrandingError {}

impl ResidenceBrand {
    /// Accept Unicode display text while keeping the public identifier ASCII.
    ///
    /// Names are plain text. Renderers still need context-appropriate escaping;
    /// this validator does not make text safe for HTML or database queries.
    pub fn new(
        identifier: &str,
        name: &str,
        destination: &str,
    ) -> Result<Self, BrandingError> {
        if !valid_identifier(identifier) {
            return Err(BrandingError::InvalidIdentifier);
        }
        let name = display_text(name).ok_or(BrandingError::InvalidName)?;
        let destination = display_text(destination).ok_or(BrandingError::InvalidDestination)?;
        Ok(Self {
            identifier: identifier.to_owned(),
            name,
            destination,
        })
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn destination(&self) -> &str {
        &self.destination
    }
}

fn valid_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_IDENTIFIER_BYTES
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.contains("--")
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'
        })
}

fn display_text(value: &str) -> Option<String> {
    // Reject controls before trimming so tabs/newlines cannot be silently hidden.
    if value.chars().any(char::is_control) {
        return None;
    }
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.chars().count() > MAX_DISPLAY_CHARACTERS {
        return None;
    }
    Some(trimmed.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn residence_brand_preserves_its_identity() {
        let brand = ResidenceBrand::new("dar-oliva", "DAR OLIVA", "Marrakech").unwrap();
        assert_eq!(brand.identifier(), "dar-oliva");
        assert_eq!(brand.name(), "DAR OLIVA");
        assert_eq!(brand.destination(), "Marrakech");
    }

    #[test]
    fn unicode_display_text_is_preserved_and_outer_spaces_are_trimmed() {
        let brand = ResidenceBrand::new("casa-01", "  Résidence du Soleil  ", " Mérida ").unwrap();
        assert_eq!(brand.name(), "Résidence du Soleil");
        assert_eq!(brand.destination(), "Mérida");
    }

    #[test]
    fn malformed_identifiers_are_refused_without_normalization() {
        for identifier in ["", "Dar-Oliva", " dar-oliva", "dar-oliva ", "-dar", "dar-",
            "dar--oliva", "dar_oliva", "dar/oliva", "dár-oliva", "dar\noliva"] {
            assert_eq!(
                ResidenceBrand::new(identifier, "Home", "Marrakech"),
                Err(BrandingError::InvalidIdentifier),
                "identifier: {identifier:?}"
            );
        }
    }

    #[test]
    fn identifier_byte_limit_is_enforced_at_the_boundary() {
        assert!(ResidenceBrand::new(&"a".repeat(MAX_IDENTIFIER_BYTES), "Home", "Tulum").is_ok());
        assert_eq!(
            ResidenceBrand::new(&"a".repeat(MAX_IDENTIFIER_BYTES + 1), "Home", "Tulum"),
            Err(BrandingError::InvalidIdentifier)
        );
    }

    #[test]
    fn empty_display_fields_have_distinct_errors() {
        assert_eq!(ResidenceBrand::new("home", "  ", "Tulum"), Err(BrandingError::InvalidName));
        assert_eq!(ResidenceBrand::new("home", "Home", "  "), Err(BrandingError::InvalidDestination));
    }

    #[test]
    fn display_limit_counts_unicode_characters_instead_of_bytes() {
        let accepted = "é".repeat(MAX_DISPLAY_CHARACTERS);
        let refused = "é".repeat(MAX_DISPLAY_CHARACTERS + 1);
        assert!(ResidenceBrand::new("home", &accepted, &accepted).is_ok());
        assert_eq!(ResidenceBrand::new("home", &refused, "Tulum"), Err(BrandingError::InvalidName));
        assert_eq!(ResidenceBrand::new("home", "Home", &refused), Err(BrandingError::InvalidDestination));
    }

    #[test]
    fn controls_are_rejected_in_both_display_fields() {
        for control in ['\n', '\r', '\t', '\0', '\u{007f}'] {
            let value = format!("Home{control}");
            assert_eq!(ResidenceBrand::new("home", &value, "Tulum"), Err(BrandingError::InvalidName));
            assert_eq!(ResidenceBrand::new("home", "Home", &value), Err(BrandingError::InvalidDestination));
        }
    }

    #[test]
    fn plain_text_is_not_confused_with_html_sanitization() {
        let brand = ResidenceBrand::new("home", "Home & <Garden>", "Tulum").unwrap();
        assert_eq!(brand.name(), "Home & <Garden>");
    }
}
