//! # KI18n-rs
//! KI18n is a cross-platform internationalization framework used by KDE applications. This crate is meant to allow using KI18n with
//! Rust and [qmetaobject-rs](https://github.com/woboq/qmetaobject-rs) crate.
//!
//! # Custom Location for KF5I18n
//! The crate searches from KF5I18n using either the environment variables (KF5_I18n_INCLUDE_PATH and KF5_I18n_LIBRARY_PATH) if they are set or just
//! searches at /usr/include /usr/library
//!
//! # Example
//! ```ignore
//! use cstr::cstr;
//! use qmetaobject::prelude::*;
//! use ki18n::KLocalizedContext;
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

pub mod klocalizedcontext;
pub mod klocalizedstring;
pub mod kcountry;

pub mod prelude {
    pub use crate::klocalizedstring::KLocalizedString;
}
