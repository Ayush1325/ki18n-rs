use cpp::{cpp, cpp_class};
use qttypes::QString;

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

    /// Adds a context for which this Translator should be active.
    pub fn add_context_to_monitor(&mut self, context: QString) {
        cpp!(unsafe [self as "KLocalizedTranslatorHolder *", context as "QString"] {
            self->translator->addContextToMonitor(context);
        });
    }

    /// Stop translating for the given context.
    pub fn remove_context_to_moitor(&mut self, context: QString) {
        cpp!(unsafe [self as "KLocalizedTranslatorHolder *", context as "QString"] {
            self->translator->removeContextToMonitor(context);
        });
    }

    /// Sets the translationDomain to be used.
    pub fn set_translation_domain(&mut self, translation_domain: QString) {
        cpp!(unsafe [self as "KLocalizedTranslatorHolder *", translation_domain as "QString"] {
            self->translator->setTranslationDomain(translation_domain);
        });
    }
}
