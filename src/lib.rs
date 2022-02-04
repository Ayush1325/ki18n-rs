//! # Introduction
//! KI18n is a cross-platform internationalization framework used by KDE applications. This crate
//! is meant to allow using KI18n with Rust and [qmetaobject-rs](https://github.com/woboq/qmetaobject-rs) crate.
//!
//! # Motivation
//! I love KDE. I have been using it as my primary DE for quite a while. I also like Rust Programming Language.
//! Currently, it is nearly impossible to use any KDE frameworks from Rust without some C++ FFI. While FFI is
//! never easy, complexity with C++ is exponentially more than plain C due to the Object-Oriented nature of C++,
//! which is entirely different from Rust's somewhat functional design.
//!
//! If possible, I would like the Tier-1 KDE Frameworks to be usable from pragmatic Rust in a somewhat natural
//! fashion. I am currently interested in KDE frameworks that are usable with QML.
//!
//! # Requirements
//! This crate requires KF5I18n to be installed or at least present in the system.
//! ## Ubuntu
//! ``` shell
//! sudo apt install libkf5i18n-dev
//! ```
//! ## Arch Linux
//! ``` shell
//! sudo pacman -S ki18n
//! ```
//!
//! # Custom Location for KF5I18n
//! The crate searches from KF5I18n using either the environment variables (`KF_VERSION`,
//! `KF_INCLUDE_PATH` and `KF_LIBRARY_PATH`) if they are set or uses `kf5-config` to find the paths.
//!
//! # Features
//! - `qmetaobject` : Enables some methods that require qmetaobject. Most people will need this.
//!
//! # Example
//! ```no-run
//! use cstr::cstr;
//! use qmetaobject::prelude::*;
//! use ki18n::klocalizedcontext::KLocalizedContext;
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
pub mod klocalizedtranslator;
// pub mod kcountry;

pub mod prelude {
    pub use crate::klocalizedcontext::KLocalizedContext;
    pub use crate::klocalizedstring::KLocalizedString;
}
