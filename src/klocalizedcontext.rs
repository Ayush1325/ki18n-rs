use cpp::{cpp, cpp_class};
use qttypes::QString;
use std::ffi::c_void;

#[cfg(feature = "qmetaobject")]
use qmetaobject::{QObject, QObjectPinned, QmlEngine};

cpp! {{
    #include <KLocalizedContext>
    #include <QtCore/QObject>
    #include <QtQml/QQmlEngine>
    #include <QtQuick/QtQuick>

    struct KLocalizedContextHolder {
        std::shared_ptr<KLocalizedContext> klocalized;

        KLocalizedContextHolder() : klocalized(new KLocalizedContext) {}
        KLocalizedContextHolder(QObject *parent) : klocalized(new KLocalizedContext(parent)) {}
    };
}}

cpp_class!(
    /// Struct representing [KLocalizedContext](https://api.kde.org/frameworks/ki18n/html/classKLocalizedContext.html). Mainly used with QML.
    #[derive(Default)]
    pub unsafe struct KLocalizedContext as "KLocalizedContextHolder"
);

impl KLocalizedContext {
    #[cfg(feature = "qmetaobject")]
    /// Construct  a new KLocalizedContext from a QObject.
    pub fn new<T: QObject + Sized>(obj: QObjectPinned<T>) -> Self {
        let obj_ptr = obj.get_or_create_cpp_object();
        cpp!(unsafe [obj_ptr as "QObject *"] -> KLocalizedContext as "KLocalizedContextHolder" {
            return KLocalizedContextHolder(obj_ptr);
        })
    }

    #[cfg(feature = "qmetaobject")]
    /// Initialize KLocalizedContext from Engine.
    /// **Feature** `qmetaobject` needs to be enabled for this function.
    /// # Example
    /// ```
    /// use ki18n::klocalizedcontext::KLocalizedContext;
    /// use qmetaobject::QmlEngine;
    ///
    /// let engine = QmlEngine::new();
    /// KLocalizedContext::init_from_engine(&engine);
    /// ```
    pub fn init_from_engine(engine: &QmlEngine) {
        let engine_ptr = engine.cpp_ptr();
        cpp!(unsafe [engine_ptr as "QQmlEngine*"]{
            engine_ptr->rootContext()->setContextObject(new KLocalizedContext(engine_ptr));
        })
    }

    /// Returns a pointer to the C++ object. The pointer is of the type `KLocalizedContext *` in C++.
    pub fn cpp_ptr(&self) -> *mut c_void {
        cpp!(unsafe [self as "const KLocalizedContextHolder *"] -> *mut c_void as "KLocalizedContext *" {
            return self->klocalized.get();
        })
    }

    /// Set Translation Domain for current KLocalizedContext.
    #[cfg_attr(
        feature = "qmetaobject",
        doc = r##"
# Example

```
use ki18n::klocalizedcontext::KLocalizedContext;
use qmetaobject::QmlEngine;

let engine = QmlEngine::new();
let mut context = KLocalizedContext::default();
context.set_translation_domain("Test Domain".into());
```
"##
    )]
    pub fn set_translation_domain(&mut self, domain: QString) {
        cpp!(unsafe [self as "KLocalizedContextHolder *", domain as "QString"] {
            self->klocalized->setTranslationDomain(domain);
        })
    }

    /// Returns the current Translation Domain.
    pub fn translation_domain(&self) -> QString {
        cpp!(unsafe [self as "const KLocalizedContextHolder *"] -> QString as "QString" {
            return self->klocalized->translationDomain();
        })
    }
}
