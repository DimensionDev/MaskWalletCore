use anyhow::Result;
use std::{
    env,
    fs::{copy, create_dir, remove_dir_all},
    process::Command,
};
use tokio::join;

use super::*;

pub async fn start_generating_xcframework() -> Result<()> {
    let output = env::current_dir()?
        .parent()
        .unwrap()
        .join(format!("output"));

    prepare_output_dir().await?;

    let prepare_dir = tokio::spawn(prepre_xcframework_dirs());
    let build_release = tokio::spawn(cargo_build_release());
    let _ = join!(prepare_dir, build_release);
    generate_xcframework().await?;
    generate_protobuf_files(output).await?;

    finish();
    Ok(())
}

async fn cargo_build_release() -> Result<()> {
    for target in vec![
        "x86_64-apple-ios",
        "aarch64-apple-ios",
        "aarch64-apple-ios-sim",
    ] {
        let _build = Command::new("cargo")
            .args(["build", "--target", target, "--release"])
            .spawn()?
            .wait_with_output();
    }

    Ok(())
}

async fn prepre_xcframework_dirs() -> Result<()> {
    let xcframework_path = env::current_dir()?
        .parent()
        .unwrap()
        .join(format!("output/ios/{:}.xcframework", FRAMEWORK));

    if xcframework_path.exists() {
        remove_dir_all(&xcframework_path)?;
    }
    create_dir(&xcframework_path)?;
    create_dir(&xcframework_path.join(format!("common")))?;
    create_dir(&xcframework_path.join(format!("common/{:}.xcframework", FRAMEWORK)))?;
    let module_path = xcframework_path.join(format!("common/{:}.xcframework/Modules", FRAMEWORK));
    create_dir(&module_path)?;

    let module_map_path = env::current_dir()?
        .as_path()
        .parent()
        .unwrap()
        .join("target-mobile/iOS/module.modulemap");
    copy(module_map_path, module_path.join("module.modulemap"))?;

    let header_path = xcframework_path.join(format!("common/{:}.xcframework/Headers", FRAMEWORK));
    create_dir(&header_path)?;
    write_header(header_path.join(format!("{:}.h", FRAMEWORK))).await?;

    Ok(())
}

async fn generate_xcframework() -> Result<()> {
    let xcframework_path = env::current_dir()?
        .parent()
        .unwrap()
        .join(format!("output/ios/{:}.xcframework", FRAMEWORK));
    let common_path = xcframework_path.join(format!("common/{:}.xcframework", FRAMEWORK));
    let arm64_path = xcframework_path.join("ios-arm64");
    let arm64_framework_path = arm64_path.join(format!("{:}.framework", FRAMEWORK));
    create_dir(&arm64_path)?;
    create_dir(&arm64_framework_path)?;
    dir_copy(&common_path, &arm64_path).await?;

    let target_path = env::current_dir()?.parent().unwrap().join("target");
    copy(
        &target_path.join(format!("aarch64-apple-ios/release/{:}", LIB_NAME)),
        &arm64_framework_path.join(format!("{:}", FRAMEWORK)),
    )?;

    let x86_arm64_sim_path = xcframework_path.join("ios-arm64_x86_64-simulator");
    let x86_arm64_sim_framework_path = x86_arm64_sim_path.join(format!("{:}.framework", FRAMEWORK));
    create_dir(&x86_arm64_sim_path)?;
    create_dir(&x86_arm64_sim_framework_path)?;

    dir_copy(&common_path, &x86_arm64_sim_path).await?;

    let _lipo_cmd = Command::new("lipo")
        .arg("-create")
        .arg("-output")
        .arg(xcframework_path.join(format!(
            "ios-arm64_x86_64-simulator/{:}.framework/{:}",
            FRAMEWORK, FRAMEWORK
        )))
        .arg(target_path.join(format!("aarch64-apple-ios-sim/release/{:}", LIB_NAME)))
        .arg(target_path.join(format!("x86_64-apple-ios/release/{:}", LIB_NAME)))
        .spawn()?
        .wait_with_output();

    copy(
        target_path
            .parent()
            .unwrap()
            .join("target-mobile/iOS/Info.plist"),
        xcframework_path.join("Info.plist"),
    )?;
    remove_dir_all(xcframework_path.join("common"))?;

    Ok(())
}
