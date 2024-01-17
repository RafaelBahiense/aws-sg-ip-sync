use users::os::unix::UserExt;
use users::{get_current_uid, get_user_by_uid};

pub fn get_home_dir_of_current_user() -> Option<String> {
    if let Some(user) = get_user_by_uid(get_current_uid()) {
        if user.uid() != 0 {
            // Not running as root, so we retrieve the home directory
            Some(user.home_dir().to_str().unwrap().to_string())
        } else {
            // Running as root
            None
        }
    } else {
        // No user found
        None
    }
}
