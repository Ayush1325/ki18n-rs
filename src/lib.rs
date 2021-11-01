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

cpp_class!(pub unsafe struct KLocalizedContext as "KLocalizedContextHolder");

impl KLocalizedContext {
    pub fn init_from_engine(engine: &QmlEngine) {
        let engine_ptr = engine.cpp_ptr();
        cpp!(unsafe [engine_ptr as "QQmlEngine*"] {
            engine_ptr->rootContext()->setContextObject(new KLocalizedContext(engine_ptr));
        });
    }
}
