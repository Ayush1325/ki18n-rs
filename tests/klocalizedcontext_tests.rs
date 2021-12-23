use ki18n::klocalizedcontext::KLocalizedContext;
use qttypes::QString;

#[cfg(feature = "qmetaobject")]
use qmetaobject::QmlEngine;

mod common;

#[test]
#[cfg(feature = "qmetaobject")]
fn cpp_ptr() {
    let _lock = common::lock_for_test();

    let engine = QmlEngine::new();
    let context = KLocalizedContext::init_from_engine(&engine);

    let context_ptr = context.cpp_ptr();
    assert_ne!(std::ptr::null(), context_ptr);
}

#[test]
#[cfg(feature = "qmetaobject")]
fn translation_domain() {
    let _lock = common::lock_for_test();

    let engine = QmlEngine::new();
    let mut context = KLocalizedContext::init_from_engine(&engine);

    const TRANSLATION_DOMAIN: &str = "Test Domain";
    context.set_translation_domain(TRANSLATION_DOMAIN.into());

    let domain = context.translation_domain();

    assert_eq!(domain, QString::from(TRANSLATION_DOMAIN));
}
