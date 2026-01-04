use dioxus_cli::Platform;

#[test]
fn test_platform_from_ohos_identifier() {
    let platform = Platform::from_identifier("ohos").unwrap();
    assert_eq!(platform, Platform::Ohos);
}

#[test]
fn test_platform_ohos_serde_roundtrip() {
    let platform = Platform::Ohos;
    let json = serde_json::to_string(&platform).unwrap();
    assert_eq!(json, "\"ohos\"");
    let decoded: Platform = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, Platform::Ohos);
}


