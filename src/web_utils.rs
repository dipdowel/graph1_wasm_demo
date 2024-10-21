use web_sys::console;

pub fn console_log(msg: &str) {
    console::log_1(&msg.into());
}
