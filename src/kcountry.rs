use cpp::{cpp, cpp_class};

cpp! {{
    #include <KCountry>
}}

cpp_class!(
    #[derive(Default, Clone, Eq, PartialEq)]
    pub unsafe struct KCountry as "KCountry"
);
