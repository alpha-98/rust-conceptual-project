# rust-conceptual-project
This project aims to demystify the concepts of Rust programming language through hands-on coding exercises and real-world examples. It targets individuals with basic programming experience who are eager to learn Rust but find its abstractions and ownership system challenging.

Rust Docs Link : https://doc.rust-lang.org/book/

## Getting Started
### Installing the Rust 
```
https://www.rust-lang.org/tools/install
``` 
**NOTE**: For Windows, SET ~/cargo/bin path to your Environment Variable's PATH

### Build Project
- Create with new Folder Or Directory
    ```
    cargo new <project_name>
    ```
- Create in Existing Folder or Directory 
    ```
    cargo init <path>
    ```
    **NOTE:** Here `path` can be a relative path or absolute path

### Run the Project : 
- **We should run all below command from our project's root directory**.
 - First build it
    ```
    cargo build
    ```
 - Second run it
    ```
    cargo run
    ```

### Explanation of Initial Project
 - The `cargo new <project_name>` command creates a new Rust Project in a directory called `<project_name>`.
 - The Directory contains several files and directories that **Cargo** uses to manage the project.
 - Including a `Cargo.toml` file that contains the **metadata** about the project.
 - `src` directory that contains the source code.    

### Cargo.toml
- This is file where we store the metadata about our project.
- This includes information such as project **name**, **version**, **authors** and **dependencies**.
- It's Written in the [TOML](https://toml.io/en/) format which is popular configuration file format in the RUST Community.
- There are two section in this file `[package]` and `[dependencies]`
    - `[package]` section contains the metadata about the project.
    - `[dependencies]` section lists external **crates** that the project depends on.
- Everything that follows a header ( such as `[package]`) is part of that section that continues until another section starts.


# Continue Working On It .....