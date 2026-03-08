use i18n_country_translations::Registry;

#[test]
fn test_register_and_lookup() {
    let mut reg = Registry::new();
    reg.register_locale("en").unwrap();
    reg.register_locale("de").unwrap();
    reg.set_default_locale("en").unwrap();

    assert_eq!(reg.get_name("US"), Some("United States"));
    assert_eq!(reg.get_name("us"), Some("United States")); // auto-uppercase
    assert!(reg.get_name_for_locale("de", "US").is_some());

    // NO must be Norway, not empty/false
    assert_eq!(reg.get_name("NO"), Some("Norway"));
}

#[test]
fn test_register_all() {
    let mut reg = Registry::new();
    reg.register_all_locales().unwrap();
    assert!(reg.registered_locales().len() >= 168);
}

#[test]
fn test_available_locales() {
    let locales = Registry::available_locales();
    assert!(locales.len() >= 168);
    assert!(locales.contains(&"en"));
    assert!(locales.contains(&"de"));
}

#[test]
fn test_global_api() {
    i18n_country_translations::register_locale("en").unwrap();
    i18n_country_translations::set_default_locale("en").unwrap();
    assert_eq!(i18n_country_translations::get_name("NO"), Some("Norway".to_string()));
}

#[test]
fn test_unregistered_locale() {
    let mut reg = Registry::new();
    assert!(reg.set_default_locale("zz").is_err());
    assert!(reg.get_name_for_locale("zz", "US").is_none());
}
