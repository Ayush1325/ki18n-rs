/* Copyright (C) 2018 Olivier Goffart <ogoffart@woboq.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
associated documentation files (the "Software"), to deal in the Software without restriction,
including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial
portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES
OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

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
    println!("cargo:rerun-if-env-changed=KF5_I18n_INCLUDE_PATH");
    println!("cargo:rerun-if-env-changed=KF5_I18n_LIBRARY_PATH");

    let (kf5i18n_include_path, kf5i18n_library_path) = match (
        std::env::var("KF5_I18n_INCLUDE_PATH").ok(),
        std::env::var("KF5_I18n_LIBRARY_PATH").ok(),
    ) {
        (Some(include_path), Some(library_path)) => (include_path, library_path),
        (None, None) => {
            const DEFAULT_INCLUDE_PATH: &str = "/usr/include/KF5/KI18n";
            (DEFAULT_INCLUDE_PATH.to_string(), "KF5I18n".to_string())
        }
        (Some(_), None) | (None, Some(_)) => {
            panic!("KF5_KI18n_INCLUDE_PATH and KF5_KI18n_LIBRARY_PATH env variable must be either both empty or both set.")
        }
    };

    config.include(kf5i18n_include_path);

    println!("cargo:rustc-link-lib={}", kf5i18n_library_path);
}
