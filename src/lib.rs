//! # KI18n-rs
//! KI18n is a cross-platform internationalization framework used by KDE applications. This crate is meant to allow using KI18n with
//! Rust and [qmetaobject-rs](https://github.com/woboq/qmetaobject-rs) crate.
//! # Example
//! ```rust
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
use cpp::{cpp, cpp_class};
use qmetaobject::prelude::*;
use qmetaobject::QObjectPinned;

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
    /// KLocalizedContext::init_from_engine(&engine);
    /// ```
    pub fn init_from_engine(engine: &QmlEngine) {
        let engine_ptr = engine.cpp_ptr();
        cpp!(unsafe [engine_ptr as "QQmlEngine*"] {
            engine_ptr->rootContext()->setContextObject(new KLocalizedContext(engine_ptr));
        });
    }
}
