# rialight::intl

Internationalization module.

## Progress

Use either the Ecma-262 Intl or the [ICU4X project](https://crates.io/crates/icu) for implementing several things.

Not all browsers implement all of Ecma-262 Intl API, or ICU4X doesn't cover all of Intl, so we'll have to support less Intl things for the time being. The goal of this crate is to implement all of Intl. Add more checkboxes for specific features here.

- [ ] Currently the `text_direction()` is incorrectly implemented. It compares the language part, not the script part. To fix that, `LocaleExpander` from `icu` provides a maximize method, but the constructor requires the data provider. https://github.com/unicode-org/icu4x/issues/3172#issuecomment-1462282871
- [ ] Include `icu` data only for non-WebAssembly target.
- [ ] Learn how to use `js!` macro from `stdweb`.
- [ ] Collator
  - [ ] Using `icu`
  - [ ] Using browser-available Ecma-262 `Intl`
- [ ] DateTimeFormat
  - [ ] Using `icu`
  - [ ] Using browser-available Ecma-262 `Intl`
- [ ] DisplayNames
  - [ ] Using `icu`. Use LanguageDisplayNames and RegionDisplayNames from [icu_displaynames](https://docs.rs/icu_displaynames/latest/icu_displaynames/index.html)
  - [ ] Using browser-available Ecma-262 `Intl`
- [ ] ListFormat
  - [ ] Using `icu`
  - [ ] Using browser-available Ecma-262 `Intl`
- [x] Locale
- [ ] NumberFormat
  - [ ] Using `icu`
  - [ ] Using browser-available Ecma-262 `Intl`
- [ ] PluralRules
  - [ ] Using `icu`
  - [ ] Using browser-available Ecma-262 `Intl`
- [ ] RelativeTimeFormat
  - [ ] Using `icu`
  - [ ] Using browser-available Ecma-262 `Intl`
- [ ] Segmenter
  - [ ] Using `icu`
  - [ ] Using browser-available Ecma-262 `Intl`

## FTL Progress

- [x] Arguments
- [x] `arguments!`
- [x] `Ftl`
  - [ ] Should the library internally use `fluent::FluentBundle::new_concurrent()`?
  - [x] `initialize_locale()`
```
ftl.initialize_locale(|locale, bundle| {
    // locale: intl::Locale
    match locale {
        _ => {},
    }
});
```