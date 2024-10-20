fn main() {
    // 获取当前时间戳
    let timestamp = std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .unwrap()
      .as_secs();
    // 设置环境变量 TEST_FOO 为当前时间戳
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    // 启用 "pass" 特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
}