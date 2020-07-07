use std::env;
use std::path;

pub struct ChangeWorkingDirectory {
    previous_directory: path::PathBuf,
}

impl ChangeWorkingDirectory {
    pub fn change(new_directory: &impl AsRef<path::Path>) -> Self {
        let current_working_directory = env::current_dir().unwrap();

        env::set_current_dir(new_directory).unwrap();

        ChangeWorkingDirectory {
            previous_directory: current_working_directory,
        }
    }
}

impl Drop for ChangeWorkingDirectory {
    fn drop(&mut self) {
        env::set_current_dir(&self.previous_directory).unwrap();
    }
}
