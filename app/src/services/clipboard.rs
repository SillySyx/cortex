pub struct ClipboardService {
}

impl ClipboardService {
    pub fn copy_to_clipboard(_value: String) {
        #[cfg(web_sys_unstable_apis)]
        {
            let window = match web_sys::window() {
                Some(value) => value,
                None => return,
            };

            let navigator = window.navigator();
            let clipboard = navigator.clipboard();

            let _ = clipboard.write_text(&_value);
        }
    }
}