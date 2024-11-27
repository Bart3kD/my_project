use std::cell::RefCell;

thread_local! {
    static MSG: RefCell<String> = RefCell::new(String::new());
}

#[ic_cdk::update]
fn save_message(message: String) {
    MSG.with(|static_message| *static_message.borrow_mut() = message);
}

#[ic_cdk::query]
fn get_message() -> String {
    MSG.with(|static_message| static_message.borrow().clone())
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}