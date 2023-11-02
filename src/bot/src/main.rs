mod admin;
mod user;
mod db;

fn main() {
    std::thread::spawn(|| {
        admin::run()
    });
    user::run();
}