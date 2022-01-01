use cpp::{cpp, cpp_class};

#[cfg(feature = "qmetaobject")]
use qmetaobject::{QObject, QObjectPinned};

cpp! {{
    #include <KLocalizedTranslator>

    struct KLocalizedTranslatorHolder {
        std::unique_ptr<KLocalizedTranslator> translator;

        KLocalizedTranslatorHolder(QObject *parent) : translator(new KLocalizedTranslator(parent)) {}
    };
}}

cpp_class!(
    /// Struct representing [KLocalizedTranslator](https://api.kde.org/frameworks/ki18n/html/classKLocalizedTranslator.html).
    pub unsafe struct KLocalizedTranslator as "KLocalizedTranslatorHolder"
);

impl KLocalizedTranslator {
    #[cfg(feature = "qmetaobject")]
    /// Initialize KLocalizedTranslator.
    pub fn new<T: QObject + Sized>(obj: QObjectPinned<T>) -> KLocalizedTranslator {
        let obj_ptr = obj.get_or_create_cpp_object();
        cpp!(unsafe [obj_ptr as "QObject *"] -> KLocalizedTranslator as "KLocalizedTranslatorHolder" {
            return KLocalizedTranslatorHolder(obj_ptr);
        })
    }
}