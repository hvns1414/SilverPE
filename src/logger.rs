use colored::Colorize;

pub fn log_ok(msg: &str) {
    // [+] Başarılı durumlar için Mor (Magenta)
    println!("{} {}", "[+]".magenta().bold(), msg);
}

pub fn log_info(msg: &str) {
    // [*] Bilgi mesajları için Mavi (Blue)
    println!("{} {}", "[*]".blue().bold(), msg);
}

pub fn log_error(msg: &str) {
    // [-] Hata mesajları için Kırmızı (Red)
    println!("{} {}", "[-]".red().bold(), msg);
}
