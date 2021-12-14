use cpp::{cpp, cpp_class};
use qmetaobject::QByteArray;
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
    /// use ki18n_rs::prelude::KLocalizedString;
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
}
