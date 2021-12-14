use ki18n_rs::klocalizedstring::KLocalizedString;
use std::ffi::CString;

#[test]
fn application_domain() {
    const APPLICATION_DOMAIN: &str = "KI18n";
    let domain = CString::new(APPLICATION_DOMAIN).unwrap();

    KLocalizedString::set_application_domain(&domain);
    let domain = KLocalizedString::application_domain();
    println!("{:#?}", domain);

    assert_eq!(domain.to_str().unwrap(), "KI18n");
}
