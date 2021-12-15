use cpp::{cpp, cpp_class};
use qmetaobject::{QByteArray, QString, QStringList};
use std::ffi::CStr;

cpp! {{
    #include <KLocalizedString>
    #include <QtCore/QByteArray>
}}

cpp_class!(
    /// Wrapper around [`KLocalizedString`][class] class
    ///
    /// [class]: https://api.kde.org/frameworks/ki18n/html/classKLocalizedString.html
    pub unsafe struct KLocalizedString as "KLocalizedString"
);

impl KLocalizedString {
    /// Set Application Domain
    /// # Example
    /// ```
    /// use std::ffi::CString;
    /// use ki18n::prelude::KLocalizedString;
    ///
    /// let domain = CString::new("Application_Domain").unwrap();
    /// KLocalizedString::set_application_domain(&domain);
    /// ```
    pub fn set_application_domain(domain: &CStr) {
        let domain_ptr = domain.as_ptr();
        cpp!(unsafe [domain_ptr as "char*"] {
            KLocalizedString::setApplicationDomain(domain_ptr);
        });
    }

    /// Get Application Domain
    pub fn application_domain() -> QByteArray {
        cpp!(unsafe [] -> QByteArray as "QByteArray" {
            return KLocalizedString::applicationDomain();
        })
    }

    /// Get the languages for which translations will be made.
    /// Returned languages are ordered with decreasing priority.
    pub fn languages() -> QStringList {
        cpp!(unsafe [] -> QStringList as "QStringList" {
            return KLocalizedString::languages();
        })
    }

    /// Set the languages for which translations will be made.
    /// This overrides the languages provided by the locale. Languages should be ordered with decreasing priority.
    /// TODO: Add Test
    pub fn set_languages(languages: &QStringList) {
        cpp!(unsafe [languages as "QStringList"] {
            KLocalizedString::setLanguages(languages);
        });
    }

    /// Clear override languages.
    /// This clears the override languages, going back to those provided by the locale.
    /// TODO: Add Test.
    pub fn clear_languages() {
        cpp!(unsafe [] {
            KLocalizedString::clearLanguages();
        })
    }

    /// Load locales for a domain from a specific location.
    /// This is useful for resources which have their translation files outside of the usual $XDG_DATA_DIRS/locales location.
    pub fn add_domain_locale_dir(domain: QByteArray, path: QString) {
        cpp!(unsafe [domain as "QByteArray", path as "QString"] {
            KLocalizedString::addDomainLocaleDir(domain, path);
        })
    }

    /// Check whether the translation catalog file in the given language for the set application translation domain exists.
    pub fn is_application_translated_into(language: QString) -> bool {
        cpp!(unsafe [language as "QString"] -> bool as "bool" {
            return KLocalizedString::isApplicationTranslatedInto(language);
        })
    }

    /// Find a path to the localized file for the given original path.
    /// This is intended mainly for non-text resources (images, sounds, etc). Text resources should be handled in more specific ways.
    pub fn localized_file_path(file_path: QString) -> QString {
        cpp!(unsafe [file_path as "QString"] -> QString as "QString" {
            return KLocalizedString::localizedFilePath(file_path);
        })
    }

    /// Remove accelerator marker from a UI text label.
    pub fn remove_accelerator_marker(label: QString) -> QString{
        cpp!(unsafe [label as "QString"] -> QString as "QString" {
            return KLocalizedString::removeAcceleratorMarker(label);
        })
    }
}
