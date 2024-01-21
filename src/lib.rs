#[cfg(target_os = "windows")]
pub fn init() {
    // static DATA_1: &'static [u8] = include_bytes!("../../wei-release/windows/san/san.txt");
    // if std::env::args().collect::<Vec<_>>().len() > 1000 {
    //     println!("{:?}", DATA_1);
    // }
}

#[cfg(not(target_os = "windows"))]
pub fn init() {
}