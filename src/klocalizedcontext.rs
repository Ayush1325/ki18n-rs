use cpp::{cpp, cpp_class};
use qmetaobject::prelude::*;
use std::ffi::c_void;

cpp! {{
    #include <KLocalizedContext>
    #include <QtCore/QObject>
    #include <QtQml/QQmlEngine>
    #include <QtQuick/QtQuick>

    struct KLocalizedContextHolder {
        std::unique_ptr<KLocalizedContext> klocalized;

        KLocalizedContextHolder(QObject *parent) : klocalized(new KLocalizedContext(parent)) {}
    };
}}

cpp_class!(
    /// Struct representing KLocalizedContext. Mainly used with QML.
    pub unsafe struct KLocalizedContext as "KLocalizedContextHolder"
);

impl KLocalizedContext {
    /// Initialize KLocalizedContext from Engine.
    pub fn init_from_engine(engine: &QmlEngine) -> Self {
        let engine_ptr = engine.cpp_ptr();
        cpp!(unsafe [engine_ptr as "QQmlEngine*"] -> KLocalizedContext as "KLocalizedContextHolder" {
            auto klocalized = new KLocalizedContext(engine_ptr);
            auto klocalizedholder = KLocalizedContextHolder(klocalized);
            engine_ptr->rootContext()->setContextObject(klocalized);
            return klocalizedholder;
        })
    }

    /// Returns a pointer to the C++ object. The pointer is of the type `KLocalizedContext *` in C++.
    pub fn cpp_ptr(&self) -> *mut c_void {
        cpp!(unsafe [self as "KLocalizedContextHolder *"] -> *mut c_void as "KLocalizedContext *" {
            return self->klocalized.get();
        })
    }

    /// Set Translation Domain for current KLocalizedContext.
    pub fn set_translation_domain(&mut self, domain: QString) {
        cpp!(unsafe [self as "KLocalizedContextHolder *", domain as "QString"] {
            self->klocalized->setTranslationDomain(domain);
        })
    }

    /// Retrns the current Translation Domain.
    pub fn translation_domain(&self) -> QString {
        cpp!(unsafe [self as "KLocalizedContextHolder *"] -> QString as "QString" {
            return self->klocalized->translationDomain();
        })
    }
}
