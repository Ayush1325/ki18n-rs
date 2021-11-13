//! # KI18n-rs
//! KI18n is a cross-platform internationalization framework used by KDE applications. This crate is meant to allow using KI18n with
//! Rust and [qmetaobject-rs](https://github.com/woboq/qmetaobject-rs) crate.
//! # Example
//! ```ignore
//! use cstr::cstr;
//! use qmetaobject::prelude::*;
//! use ki18n_rs::KLocalizedContext;
//!
//! fn main() {
//!   let mut engine = QmlEngine::new();
//!   KLocalizedContext::init_from_engine(&engine);
//!   engine.load_data(r#"
//!     import QtQuick 2.6
//!     import QtQuick.Controls 2.0 as Controls
//!     import QtQuick.Layouts 1.2
//!     import org.kde.kirigami 2.13 as Kirigami
//!     
//!     // Base element, provides basic features needed for all kirigami applications
//!     Kirigami.ApplicationWindow {
//!         // ID provides unique identifier to reference this element
//!         id: root
//!     
//!         // Window title
//!         // i18nc is useful for adding context for translators, also lets strings be changed for different languages
//!         title: i18nc("@title:window", "Hello World")
//!     
//!         // Initial page to be loaded on app load
//!         pageStack.initialPage: Kirigami.Page {
//!     
//!             Controls.Label {
//!                 // Center label horizontally and vertically within parent element
//!                 anchors.centerIn: parent
//!                 text: i18n("Hello World!")
//!             }
//!         }
//!     }
//!   "#.into());
//!   engine.exec();
//! }
//! ```
use qmetaobject::prelude::*;
use std::ffi::c_void;

#[cfg(not(no_qt))]
use cpp::{cpp, cpp_class};

#[cfg(no_qt)]
mod no_qt {
    pub fn panic<T>() -> T {
        panic!("This example is not supported on Qt 6 and above")
    }
}

#[cfg(no_qt)]
macro_rules! cpp {
    {{ $($t:tt)* }} => {};
    {$(unsafe)? [$($a:tt)*] -> $ret:ty as $b:tt { $($t:tt)* } } => {
        crate::no_qt::panic::<$ret>()
    };
    { $($t:tt)* } => {
        crate::no_qt::panic::<()>()
    };
}

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
    /// ```rust
    /// use qmetaobject::prelude::*;
    /// use ki18n_rs::KLocalizedContext;
    ///
    /// let mut engine = QmlEngine::new();
    /// let context = KLocalizedContext::init_from_engine(&engine);
    /// ```
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
    /// ```rust
    /// use qmetaobject::prelude::*;
    /// use ki18n_rs::KLocalizedContext;
    ///
    /// let mut engine = QmlEngine::new();
    /// let context = KLocalizedContext::init_from_engine(&engine);
    ///
    /// let context_ptr = context.cpp_ptr();
    /// ```
    pub fn cpp_ptr(&self) -> *mut c_void {
        cpp!(unsafe [self as "KLocalizedContextHolder *"] -> *mut c_void as "KLocalizedContext *" {
            return self->klocalized.get();
        })
    }

    /// Set Translation Domain for current KLocalizedContext.
    /// ```rust
    /// use qmetaobject::prelude::*;
    /// use ki18n_rs::KLocalizedContext;
    ///
    /// let mut engine = QmlEngine::new();
    /// let mut context = KLocalizedContext::init_from_engine(&engine);
    ///
    /// const TRANSLATION_DOMAIN: &str = "Test Domain";
    /// context.set_translation_domain(TRANSLATION_DOMAIN.into());
    ///
    /// let domain = context.translation_domain();
    ///
    /// assert_eq!(domain, QString::from(TRANSLATION_DOMAIN));
    /// ```
    pub fn set_translation_domain(&mut self, domain: QString) {
        cpp!(unsafe [self as "KLocalizedContextHolder *", domain as "QString"] {
            self->klocalized->setTranslationDomain(domain);
        })
    }

    /// Retrns the current Translation Domain.
    /// ```rust
    /// use qmetaobject::prelude::*;
    /// use ki18n_rs::KLocalizedContext;
    ///
    /// let mut engine = QmlEngine::new();
    /// let mut context = KLocalizedContext::init_from_engine(&engine);
    ///
    /// let domain = context.translation_domain();
    ///
    /// assert_eq!(domain, QString::from(""));
    /// ```
    pub fn translation_domain(&self) -> QString {
        cpp!(unsafe [self as "KLocalizedContextHolder *"] -> QString as "QString" {
            return self->klocalized->translationDomain();
        })
    }
}
