# chwd [![Build Status](https://app.travis-ci.com/GrumpyMetalGuy/chwd.svg?branch=master)](https://app.travis-ci.com/GrumpyMetalGuy/chwd) [![Documentation](https://docs.rs/chwd/badge.svg)](https://docs.rs/chwd)
Rust library to temporarily change your cwd, then switch it back when you're done.

## Usage
Call ChangeWorkingDirectory's change function to change the current working directory, assigning the result to a local temporary variable. Once this variable goes out of scope, the current working directory will change back to the directory that was in use at the time of object creation.
```rust,no_run
fn main()
{
    {
        let _dir_change = ChangeWorkingDirectory::change(&env::temp_dir())?;
        // Do something in the temp dir
    }

    // _dir_change has gone out of scope, you will be back where you started.
}
```
## Contributions
Although this is a fairly simple library, if you can think of anything that could be done to improve it, please open an issue or submit a PR!
## License

This project is licensed under

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)