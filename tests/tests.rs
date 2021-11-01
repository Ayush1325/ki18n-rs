use ki18n_rs::*;
use qmetaobject::prelude::*;

#[test]
fn test_klocalized_init() {
    let mut engine = QmlEngine::new();
    KLocalizedContext::init_from_engine(&engine);
}
