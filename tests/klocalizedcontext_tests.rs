use ki18n_rs::klocalizedcontext::KLocalizedContext;
use qmetaobject::prelude::*;
mod common;

#[test]
fn init_engine() {
    let _lock = common::lock_for_test();

    let engine = QmlEngine::new();
    KLocalizedContext::init_from_engine(&engine);
}

#[test]
fn cpp_ptr() {
    let _lock = common::lock_for_test();

    let engine = QmlEngine::new();
    let context = KLocalizedContext::init_from_engine(&engine);

    let context_ptr = context.cpp_ptr();
    assert_ne!(std::ptr::null(), context_ptr);
}

#[test]
fn translation_domain() {
    let _lock = common::lock_for_test();

    let engine = QmlEngine::new();
    let mut context = KLocalizedContext::init_from_engine(&engine);

    const TRANSLATION_DOMAIN: &str = "Test Domain";
    context.set_translation_domain(TRANSLATION_DOMAIN.into());

    let domain = context.translation_domain();

    assert_eq!(domain, QString::from(TRANSLATION_DOMAIN));
}
