pub mod conf;
pub mod docker;
pub mod file;
pub mod nginx;
pub mod service;
pub mod targz;

// logs errors and exits
#[macro_export]
macro_rules! log_exit {
    ($($x:expr),+) => {
        {
            $(eprintln!("[ERROR] {}", $x);)+
            std::process::exit(1)
        }
    };
}
