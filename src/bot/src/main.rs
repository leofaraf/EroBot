mod admin;
mod user;
mod db;
mod controller;

fn main() {
    std::thread::spawn(|| {
        admin::run()
    });
    std::thread::spawn(|| {
        controller::run()
    });
    user::run();
}