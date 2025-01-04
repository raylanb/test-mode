fn main() {
    // 1. 调用 tauri_build 的构建脚本
    tauri_build::build();

    // 2. 设置移动端需要的环境变量
    println!("cargo:rustc-env=TAURI_ANDROID_PACKAGE_NAME_PREFIX=com.test-mode.app");
    println!("cargo:rustc-env=TAURI_ANDROID_PACKAGE_NAME_APP_NAME=test-mode");
}