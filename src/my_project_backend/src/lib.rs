use std::cell::RefCell;

thread_local! {
    static CHAT: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
fn save_message(message: String) {
    CHAT.with(|static_message| static_message.borrow_mut().push(message));
}

#[ic_cdk::query]
fn get_chat() -> Vec<String> {
    CHAT.with(|static_message| static_message.borrow().clone())
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}