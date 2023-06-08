use std::process::Command;

use android_manifest::UsesSdk;

use crate::{
    error::*,
    types::{AndroidNdk, AndroidSdk},
};

use super::gradle_init::gradle_init;

pub fn replace_config_gradle(
    content: &str,
    ndk: &AndroidNdk,
    sdk: &AndroidSdk,
    uses_sdk: Option<UsesSdk>,
) -> Result<String> {
    let uses_sdk = uses_sdk.unwrap_or_default();
    let new_content = content
        .replace(
            "{{MIN_SDK}}",
            &uses_sdk.min_sdk_version.unwrap_or(19).to_string(),
        )
        .replace(
            "{{TARGET_SDK}}",
            &uses_sdk.target_sdk_version.unwrap_or(19).to_string(),
        )
        .replace("{{GRADLE_VERSION}}", &get_gradle_version()?)
        .replace("{{BUILD_TOOLS}}", sdk.build_deps_version())
        .replace("{{JAVA_VERSION}}", &get_java_version()?)
        .replace("{{NDK_VERSION}}", ndk.version());

    Ok(new_content)
}

fn get_gradle_version() -> Result<String> {
    if let Ok(gradle_version) = std::env::var("GRADLE_VERSION") {
        return Ok(gradle_version);
    }

    if let Ok(mut gradle_cmd) = gradle_init() {
        gradle_cmd.arg("--version");
        let raw_content = gradle_cmd.output_err(true)?;
        let raw_content = String::from_utf8(raw_content.stdout).unwrap();
        for line in raw_content.lines() {
            if line.starts_with("Gradle ") {
                return Ok(line.replace("Gradle ", "").trim().to_string());
            }
        }
    }

    Ok("7.6.1".to_string())
}

fn get_java_version() -> Result<String> {
    let mut java_cmd = if let Ok(gradle) = which::which(bat!("java")) {
        Command::new(gradle)
    } else {
        let java_home = std::env::var("JAVA_HOME").ok();
        let gradle_path = std::path::PathBuf::from(java_home.ok_or(AndroidError::JavaNotFound)?);
        let gradle_executable = gradle_path.join("bin").join(bat!("java"));
        Command::new(gradle_executable)
    };
    java_cmd.arg("--version");
    let raw_content = java_cmd.output_err(true)?;
    let raw_content = String::from_utf8(raw_content.stdout).unwrap();
    let raw_content = raw_content.splitn(2, "version \"");
    let raw_content = raw_content.skip(1).next().unwrap_or("11.0.18");
    Ok(raw_content
        .splitn(2, '.')
        .next()
        .map(|v| v.to_string())
        .unwrap_or("11".to_string()))
}
