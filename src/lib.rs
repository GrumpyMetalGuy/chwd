use std::env;
use std::path;

#[derive(Debug)]
pub struct ChangeWorkingDirectory {
    previous_directory: path::PathBuf,
}

impl ChangeWorkingDirectory {
    /// Store the current working directory, then change it to the supplied path. When the struct
    /// goes out of scope, the current working directory will be changed back to what it originally
    /// was.
    ///
    /// ```rust
    /// use std::env;
    /// use chsh::ChangeWorkingDirectory;
    ///
    /// fn main()
    /// {
    ///     {
    ///         let _dir_change = ChangeWorkingDirectory::change(&env::temp_dir());
    ///         // Do something in the temp dir
    ///     }
    ///
    ///     // _dir_change has gone out of scope, you will be back where you started.
    /// }
    /// ```
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
