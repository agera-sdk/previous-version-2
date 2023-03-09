#![feature(decl_macro)]

pub mod locale;
pub use locale::Locale;

pub use fluent;
pub mod ftl;

/// Represents a language's text reading direction.
#[derive(Copy, Clone, PartialEq)]
pub enum TextDirection {
    Ltr,
    Rtl,
}

/// Trait for types that implement a `text_direction()` property.
pub trait HasTextDirection {
    fn text_direction(&self) -> TextDirection;
}

impl HasTextDirection for locale::Locale {
    fn text_direction(&self) -> TextDirection {
        self.id.text_direction()
    }
}

impl HasTextDirection for locale::LanguageIdentifier {
    fn text_direction(&self) -> TextDirection {
        self.language.text_direction()
    }
}

impl HasTextDirection for locale::subtags::Language {
    fn text_direction(&self) -> TextDirection {
        match self.as_str() {
            | "ar" | "ara"
            | "arc"
            | "az" | "aze"
            | "dv" | "div"
            | "he" | "heb"
            | "ku" | "kur"
            | "fa" | "per" | "fas"
            | "ur" | "urd"
                => TextDirection::Rtl,
            _ => TextDirection::Ltr,
        }
    }
}