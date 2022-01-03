use semver::Version;

fn main() {
    eprintln!("cargo:warning={:?}", std::env::vars().collect::<Vec<_>>());

    let mut config = cpp_build::Config::new();

    let qt_version = qt_setup(&mut config);
    ki18n_setup(&mut config);

    config.build("src/lib.rs");

    for minor in 7..=15 {
        if qt_version >= Version::new(5, minor, 0) {
            println!("cargo:rustc-cfg=qt_{}_{}", 5, minor);
        }
    }
    let mut minor = 0;
    while qt_version >= Version::new(6, minor, 0) {
        println!("cargo:rustc-cfg=qt_{}_{}", 6, minor);
        minor += 1;
    }
}

fn qt_setup(config: &mut cpp_build::Config) -> Version {
    let qt_include_path = std::env::var("DEP_QT_INCLUDE_PATH").unwrap();
    let qt_library_path = std::env::var("DEP_QT_LIBRARY_PATH").unwrap();
    let qt_version = std::env::var("DEP_QT_VERSION")
        .unwrap()
        .parse::<Version>()
        .expect("Parsing Qt version failed");

    if cfg!(target_os = "macos") {
        config.flag("-F");
        config.flag(&qt_library_path);
    }

    if qt_version >= Version::new(6, 0, 0) {
        config.flag_if_supported("-std=c++17");
        config.flag_if_supported("/std:c++17");
        config.flag_if_supported("/Zc:__cplusplus");
    }

    config.include(&qt_include_path);

    // Include qtcore
    config.include(&format!("{}/{}", qt_include_path, "QtCore"));

    qt_version
}

fn ki18n_setup(config: &mut cpp_build::Config) {
    const LIB_NAME: &str = "I18n";

    config.include(kde_frameworks::get_lib_include_path(LIB_NAME).unwrap());

    kde_frameworks::link_lib(LIB_NAME).unwrap();
}
