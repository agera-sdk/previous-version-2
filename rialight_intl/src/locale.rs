//! Parsing, manipulating, and serializing Unicode Language and Locale Identifiers.
//!
//! The module provides algorithms for parsing a string into a well-formed language or locale identifier
//! as defined by [`UTS #35: Unicode LDML 3. Unicode Language and Locale Identifiers`].
//!
//! [`Locale`] is the most common structure to use for storing information about a language,
//! script, region, variants and extensions. In almost all cases, this struct should be used as the
//! base unit for all locale management operations.
//!
//! [`LanguageIdentifier`] is a strict subset of [`Locale`] which can be useful in a narrow range of
//! cases where [`Unicode Extensions`] are not relevant.
//!
//! If in doubt, use [`Locale`].
//!
//! # Examples
//!
//! ```
//! use rialight::intl::locale::{
//!     Locale,
//!     locale, subtags_language as language, subtags_region as region,
//! };
//!
//! let mut loc: Locale = locale!("en-US");
//!
//! assert_eq!(loc.id.language, language!("en"));
//! assert_eq!(loc.id.script, None);
//! assert_eq!(loc.id.region, Some(region!("US")));
//! assert_eq!(loc.id.variants.len(), 0);
//!
//! loc.id.region = Some(region!("GB"));
//!
//! assert_eq!(loc, locale!("en-GB"));
//! ```
//!
//! For more details, see [`Locale`] and [`LanguageIdentifier`].
//!
//! [`UTS #35: Unicode LDML 3. Unicode Language and Locale Identifiers`]: https://unicode.org/reports/tr35/tr35.html#Unicode_Language_and_Locale_Identifiers
//! [`ICU4X`]: ../icu/index.html
//! [`Unicode Extensions`]: extensions

pub use icu::locid::*;