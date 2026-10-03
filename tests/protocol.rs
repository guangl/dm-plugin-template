use std::process::Command;

#[test]
fn direct_execution_requires_host_protocol() {
    let output = Command::new(env!("CARGO_BIN_EXE_dm-hello"))
        .env_remove("DM_PLUGIN_API_VERSION")
        .output()
        .unwrap();
    assert!(!output.status.success());
}

#[test]
fn host_protocol_passes_arguments_to_plugin() {
    let home = std::env::temp_dir().join("dm-hello-template-test");
    let output = Command::new(env!("CARGO_BIN_EXE_dm-hello"))
        .env("DM_PLUGIN_API_VERSION", "1")
        .env("DM_PLUGIN_CAPABILITIES", "config-dirs-v1")
        .env("DM_PLUGIN_HOME", &home)
        .env("DM_PLUGIN_DIR", home.join("plugin"))
        .env("DM_PLUGIN_CONFIG_DIR", home.join("config"))
        .env("DM_PLUGIN_DATA_DIR", home.join("data"))
        .env("DM_PLUGIN_CACHE_DIR", home.join("cache"))
        .arg("world")
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8(output.stdout).unwrap().contains("world"));
}
