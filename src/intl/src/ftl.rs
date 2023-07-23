//! Module for managing Fluent Translation List (FTL).
//!
//! # FTL Syntax
//!
//! [See the FTL syntax guide.](https://projectfluent.org/fluent/guide/)

pub use fluent::FluentArgs as Arguments;

use icu::locid::Locale;
use std::{
    cell::{Cell}, collections::{HashMap, HashSet}, sync::{Arc, RwLock},
};
use rialight_util::{hashmap, hashset};

/// Creates an `Arguments` object from a list of key-value pairs.
///
/// ## Example
///
/// ```
/// use rialight::intl;
///
/// let a = intl::ftl::arguments!{
///     "a" => "foo",
///     "b" => "bar",
/// };
/// ```
pub macro arguments {
    ($($key:expr => $value:expr,)+) => {
        {
            #[allow(unused_mut)]
            let mut r_map = ::fluent::FluentArgs::new();
            $(
                let _ = r_map.set($key.to_string(), Box::new($value));
            )*
            r_map
        }
    },
    ($($key:expr => $value:expr),*) => {
        {
            #[allow(unused_mut)]
            let mut r_map = ::fluent::FluentArgs::new();
            $(
                let _ = r_map.set($key.to_string(), Box::new($value));
            )*
            r_map
        }
    }
}

/// Interface for working with Fluent Translation Lists.
pub struct Ftl {
    m_current_locale: RwLock<Option<Locale>>,
    /// Maps a Locale object to its equivalent path component.
    /// The string to which the Locale maps depends in how the
    /// Ftl object was constructed. If the `supported_locales` option
    /// contains "en-us", then `m_locale_to_path_components.get(&locale!("en-US"))` returns "en-us".
    /// When FTLs are loaded, this component is appended to the URL or file path;
    /// for example, `"res/lang/en-us"`.
    m_locale_to_path_components: Arc<HashMap<Locale, String>>,
    m_supported_locales: Arc<HashSet<Locale>>,
    m_default_locale: Locale,
    m_fallbacks: Arc<HashMap<Locale, Vec<Locale>>>,
    m_locale_initializers: Arc<RwLock<Vec<fn(Locale, Arc<fluent::FluentBundle<fluent::FluentResource>>)>>>,
    m_assets: Arc<RwLock<HashMap<Locale, Arc<fluent::FluentBundle<fluent::FluentResource>>>>>,
    m_assets_source: String,
    m_assets_files: Vec<String>,
    m_assets_clean_unused: bool,
    m_assets_load_method: FtlLoadMethod,
}

fn parse_locale_or_panic(s: &str) -> Locale {
    Locale::try_from_bytes(s.as_bytes()).expect((format!("{} is a malformed locale.", s)).as_ref())
}

fn locale_to_unic_langid_impl_langid(locale: &Locale) -> unic_langid_impl::LanguageIdentifier {
    unic_langid_impl::LanguageIdentifier::from_bytes(locale.id.to_string().as_bytes()).unwrap()
}

fn add_ftl_bundle_resource(file_name: String, source: String, bundle: &mut fluent::FluentBundle<fluent::FluentResource>) -> bool {
    match fluent::FluentResource::try_new(source) {
        Ok(res) => {
            if let Err(error_list) = bundle.add_resource(res) {
                for e in error_list {
                    println!("Error at {}.ftl: {}", file_name, e.to_string());
                }
                return false;
            }
        },
        Err((_, error_list)) => {
            for e in error_list {
                println!("Syntax error at {}.ftl: {}", file_name, e);
            }
            return false;
        },
    }
    true
}

impl Ftl {
    /// Constructs a `Ftl` object.
    pub fn new(options: &mut FtlOptions) -> Self {
        let mut locale_to_path_components = HashMap::<Locale, String>::new();
        let mut supported_locales = HashSet::<Locale>::new();
        for unparsed_locale in options.m_supported_locales.get_mut().unwrap().iter() {
            let parsed_locale = parse_locale_or_panic(unparsed_locale);
            locale_to_path_components.insert(parsed_locale.clone(), unparsed_locale.clone());
            supported_locales.insert(parsed_locale);
        }
        let mut fallbacks = HashMap::<Locale, Vec<Locale>>::new();
        for (k, v) in options.m_fallbacks.get_mut().unwrap().iter() {
            fallbacks.insert(parse_locale_or_panic(k), v.iter().map(|s| parse_locale_or_panic(s)).collect());
        }
        let default_locale = options.m_default_locale.get_mut().unwrap().clone();
        Self {
            m_current_locale: RwLock::new(None),
            m_locale_to_path_components: Arc::new(locale_to_path_components),
            m_supported_locales: Arc::new(supported_locales),
            m_default_locale: parse_locale_or_panic(&default_locale),
            m_fallbacks: Arc::new(fallbacks),
            m_locale_initializers: Arc::new(RwLock::new(vec![])),
            m_assets: Arc::new(RwLock::new(HashMap::new())),
            m_assets_source: options.m_assets.get_mut().unwrap().m_source.get_mut().unwrap().clone(),
            m_assets_files: options.m_assets.get_mut().unwrap().m_files.get_mut().unwrap().iter().map(|s| s.clone()).collect(),
            m_assets_clean_unused: options.m_assets.get_mut().unwrap().m_clean_unused.get(),
            m_assets_load_method: options.m_assets.get_mut().unwrap().m_load_method.get(),
        }
    }

    /// Returns a set of supported locales, reflecting
    /// the ones that were specified when constructing the `Ftl` object.
    pub fn supported_locales(&self) -> HashSet<Locale> {
        self.m_supported_locales.as_ref().clone()
    }

    /// Returns `true` if the locale is one of the supported locales
    /// that were specified when constructing the `Ftl` object,
    /// otherwise `false`.
    pub fn supports_locale(&self, arg: &Locale) -> bool {
        self.m_supported_locales.contains(arg)
    }

    /// Returns the currently loaded locale.
    pub fn current_locale(&self) -> Option<Locale> {
        self.m_current_locale.read().unwrap().clone()
    }

    /// Returns the currently loaded locale followed by its fallbacks or empty if no locale is loaded.
    pub fn locale_and_fallbacks(&self) -> HashSet<Locale> {
        if let Some(c) = self.current_locale() {
            let mut r: HashSet<Locale> = hashset![c.clone()];
            self.enumerate_fallbacks(c.clone(), &mut r);
            return r;
        }
        hashset![]
    }

    /// Returns the currently loaded fallbacks.
    pub fn fallbacks(&self) -> HashSet<Locale> {
        if let Some(c) = self.current_locale() {
            let mut r: HashSet<Locale> = hashset![];
            self.enumerate_fallbacks(c.clone(), &mut r);
            return r;
        }
        hashset![]
    }

    /// Adds a callback function to initialize the `FluentBundle` object of a locale.
    /// The callback is called when the locale is loaded.
    pub fn initialize_locale(&self, callback: fn(Locale, Arc<fluent::FluentBundle<fluent::FluentResource>>)) {
        self.m_locale_initializers.write().unwrap().push(callback);
    }

    /// Attempts to load a locale and its fallbacks.
    /// If the locale argument is specified, it is loaded.
    /// Otherwise, if there is a default locale, it is loaded, and if not,
    /// the method panics.
    ///
    /// If any resource fails to load, the method returns `false`, otherwise `true`.
    pub async fn load(&self, mut new_locale: Option<Locale>) -> bool {
        if new_locale.is_none() {
            new_locale = Some(self.m_default_locale.clone());
        }
        let new_locale = new_locale.unwrap();
        if !self.supports_locale(&new_locale) {
            panic!("Unsupported locale: {}", new_locale);
        }
        let mut to_load: HashSet<Locale> = hashset![new_locale.clone()];
        self.enumerate_fallbacks(new_locale.clone(), &mut to_load);

        let mut new_assets: HashMap<Locale, Arc<fluent::FluentBundle<fluent::FluentResource>>> = hashmap![];
        for locale in to_load {
            let res = self.load_single_locale(&locale).await;
            if res.is_none() {
                return false;
            }
            new_assets.insert(locale.clone(), res.unwrap());
        }
        if self.m_assets_clean_unused {
            self.m_assets.write().unwrap().clear();
        }

        for (locale, bundle) in new_assets {
            self.m_assets.write().unwrap().insert(locale, bundle.clone());
        }
        *self.m_current_locale.write().unwrap() = Some(new_locale.clone());
        for c in self.m_locale_initializers.read().unwrap().iter() {
            c(new_locale.clone(), self.m_assets.read().unwrap()[&new_locale.clone()].clone());
        }

        true
    }

    async fn load_single_locale(&self, locale: &Locale) -> Option<Arc<fluent::FluentBundle<fluent::FluentResource>>> {
        let mut r = fluent::FluentBundle::new(vec![locale_to_unic_langid_impl_langid(locale)]);
        match self.m_assets_load_method {
            FtlLoadMethod::FileSystem => {
                for file_name in self.m_assets_files.iter() {
                    let locale_path_comp = self.m_locale_to_path_components.get(locale);
                    if locale_path_comp.is_none() {
                        panic!("Fallback is not supported a locale: {}", locale.to_string());
                    }
                    let res_path = format!("{}/{}/{}.ftl", self.m_assets_source, locale_path_comp.unwrap(), file_name);
                    let source = rialight_filesystem::File::new(res_path.clone()).read_bytes();
                    if source.is_err() {
                        println!("Failed to load resource at {}.", res_path);
                        return None;
                    }
                    let source = String::from_utf8(source.unwrap()).unwrap();
                    if !add_ftl_bundle_resource(file_name.clone(), source, &mut r) {
                        return None;
                    }
                }
            },
            FtlLoadMethod::Http => {
                for file_name in self.m_assets_files.iter() {
                    let locale_path_comp = self.m_locale_to_path_components.get(locale);
                    if locale_path_comp.is_none() {
                        panic!("Fallback is not supported a locale: {}", locale.to_string());
                    }
                    let res_path = format!("{}/{}/{}.ftl", self.m_assets_source, locale_path_comp.unwrap(), file_name);
                    let source = reqwest::get(reqwest::Url::parse(res_path.clone().as_ref()).unwrap()).await;
                    if source.is_err() {
                        println!("Failed to load resource at {}.", res_path);
                        return None;
                    }
                    let source = source.unwrap().text().await;
                    if source.is_err() {
                        println!("Failed to load resource at {}.", res_path);
                        return None;
                    }
                    let source = source.unwrap();
                    if !add_ftl_bundle_resource(file_name.clone(), source, &mut r) {
                        return None;
                    }
                }
            },
        }
        Some(Arc::new(r))
    }

    fn enumerate_fallbacks(&self, locale: Locale, output: &mut HashSet<Locale>) {
        for list in self.m_fallbacks.get(&locale).iter() {
            for item in list.iter() {
                output.insert(item.clone());
                self.enumerate_fallbacks(item.clone(), output);
            }
        }
    }

    pub fn get_message(&self, id: &str, args: Option<&Arguments>, errors: &mut Vec<fluent::FluentError>) -> Option<String> {
        self.get_message_by_locale(id, self.m_current_locale.read().unwrap().clone()?, args, errors)
    }

    fn get_message_by_locale(&self, id: &str, locale: Locale, args: Option<&Arguments>, errors: &mut Vec<fluent::FluentError>) -> Option<String> {
        if let Some(assets) = self.m_assets.read().unwrap().get(&locale) {
            if let Some(message) = assets.get_message(id) {
                return Some(self.format_pattern(message.value()?, args, errors));
            }
        }

        let fallbacks = self.m_fallbacks.get(&locale);
        if fallbacks.is_some() {
            for fl in fallbacks.unwrap().iter() {
                let r = self.get_message_by_locale(id, fl.clone(), args, errors);
                if r.is_some() {
                    return r;
                }
            }
        }
        None
    }

    pub fn has_message(&self, id: &str) -> bool {
        let locale = self.m_current_locale.read().unwrap().clone();
        if locale.is_none() {
            return false;
        }
        self.has_message_by_locale(id, locale.unwrap())
    }

    fn has_message_by_locale(&self, id: &str, locale: Locale) -> bool {
        let assets = self.m_assets.read().unwrap();
        let assets = assets.get(&locale);
        if assets.is_some() {
            if assets.unwrap().has_message(id) {
                return true;
            }
        }

        let fallbacks = self.m_fallbacks.get(&locale);
        if fallbacks.is_some() {
            for fl in fallbacks.unwrap().iter() {
                let r = self.has_message_by_locale(id, fl.clone());
                if r {
                    return true;
                }
            }
        }
        false
    }

    pub fn format_pattern(&self, pattern: &fluent_syntax::ast::Pattern<&str>, args: Option<&Arguments>, errors: &mut Vec<fluent::FluentError>) -> String {
        let locale = self.m_current_locale.read().unwrap().clone();
        if locale.is_none() {
            return "".to_owned();
        }
        let asset = &self.m_assets.read().unwrap()[&locale.unwrap()];
        asset.format_pattern(pattern, args, errors).into_owned().to_owned()
    }
}

impl Clone for Ftl {
    fn clone(&self) -> Self {
        Self {
            m_current_locale: RwLock::new(self.m_current_locale.read().unwrap().clone()),
            m_locale_to_path_components: self.m_locale_to_path_components.clone(),
            m_supported_locales: self.m_supported_locales.clone(),
            m_default_locale: self.m_default_locale.clone(),
            m_fallbacks: self.m_fallbacks.clone(),
            m_locale_initializers: self.m_locale_initializers.clone(),
            m_assets: self.m_assets.clone(),
            m_assets_source: self.m_assets_source.clone(),
            m_assets_files: self.m_assets_files.clone(),
            m_assets_clean_unused: self.m_assets_clean_unused,
            m_assets_load_method: self.m_assets_load_method,
        }
    }
}

/// Options given to the Ftl constructor.
pub struct FtlOptions {
    m_default_locale: RwLock<String>,
    m_supported_locales: RwLock<Vec<String>>,
    m_fallbacks: RwLock<HashMap<String, Vec<String>>>,
    m_assets: RwLock<FtlOptionsForAssets>,
}

impl FtlOptions {
    pub fn new() -> Self {
        FtlOptions {
            m_default_locale: RwLock::new("en".to_string()),
            m_supported_locales: RwLock::new(vec!["en".to_string()]),
            m_fallbacks: RwLock::new(hashmap! {}),
            m_assets: RwLock::new(FtlOptionsForAssets::new()),
        }
    }

    pub fn default_locale(&mut self, value: impl AsRef<str>) -> &mut Self {
        *self.m_default_locale.write().unwrap() = value.as_ref().to_owned();
        self
    }

    pub fn supported_locales(&mut self, list: Vec<impl AsRef<str>>) -> &mut Self {
        *self.m_supported_locales.write().unwrap() = list.iter().map(|name| name.as_ref().to_owned()).collect();
        self
    }

    pub fn fallbacks(&mut self, map: HashMap<impl AsRef<str>, Vec<impl AsRef<str>>>) -> &mut Self {
        *self.m_fallbacks.write().unwrap() = map.iter().map(|(k, v)| (
            k.as_ref().to_owned(),
            v.iter().map(|s| s.as_ref().to_owned()).collect()
        )).collect();
        self
    }

    pub fn assets(&mut self, options: &FtlOptionsForAssets) -> &mut Self {
        *self.m_assets.write().unwrap() = options.clone();
        self
    }
}

pub struct FtlOptionsForAssets {
    m_source: RwLock<String>,
    m_files: RwLock<Vec<String>>,
    m_clean_unused: Cell<bool>,
    m_load_method: Cell<FtlLoadMethod>,
}

impl Clone for FtlOptionsForAssets {
    fn clone(&self) -> Self {
        Self {
            m_source: RwLock::new(self.m_source.read().unwrap().clone()),
            m_files: RwLock::new(self.m_files.read().unwrap().clone()),
            m_clean_unused: self.m_clean_unused.clone(),
            m_load_method: self.m_load_method.clone(),
        }
    }
}

impl FtlOptionsForAssets {
    pub fn new() -> Self {
        FtlOptionsForAssets {
            m_source: RwLock::new("res/lang".to_string()),
            m_files: RwLock::new(vec![]),
            m_clean_unused: Cell::new(true),
            m_load_method: Cell::new(FtlLoadMethod::Http),
        }
    }
    
    pub fn source(&mut self, src: impl AsRef<str>) -> &mut Self {
        *self.m_source.write().unwrap() = src.as_ref().to_owned();
        self
    } 

    pub fn files(&mut self, list: Vec<impl AsRef<str>>) -> &mut Self {
        *self.m_files.write().unwrap() = list.iter().map(|name| name.as_ref().to_owned()).collect();
        self
    }

    pub fn clean_unused(&mut self, value: bool) -> &mut Self {
        self.m_clean_unused.set(value);
        self
    }

    pub fn load_method(&mut self, value: FtlLoadMethod) -> &mut Self {
        self.m_load_method.set(value);
        self
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum FtlLoadMethod {
    FileSystem,
    Http,
}