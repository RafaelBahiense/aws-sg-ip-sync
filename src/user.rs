use uzers::os::unix::UserExt;
use uzers::{get_current_uid, get_user_by_uid};

pub fn get_current_home_dir() -> Option<String> {
    get_user_by_uid(get_current_uid()).and_then(|user| {
        if user.uid() != 0 {
            // Not running as root, so we retrieve the home directory
            user.home_dir().to_str().map(String::from)
        } else {
            // Running as root
            None
        }
    })
}
