# Setup your Integrated Development Environment

> While teaching, the instructor will use [Microsoft Visual Studio Code](https://code.visualstudio.com/download). It's free, runs the same on Windows, Mac and Linux, and integrates well with a Rust environment.

## Extensions You Need

* [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
    * Provides everything from syntax highlighting to inline error checking, macro expansion and debugger integration.
    * Part of the core RustUp distribution.
    * Regularly Updated.
* [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
    * Integrates well with Rust-Analyzer.
    * Provides inline debugging.

## Extensions You Don't Need - But Probably Want

* [Crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates)
    * Let's you know when dependencies are outdated.
* [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml)
    * Makes editing Rust's TOML configuration easier

## Some Configuration Tweaks

> Live demo

### Enable Breakpoints Everywhere
    * Open Settings (`crtl` + `comma`)
    * Search for "everywhere".
    * Check "Allow Setting Breakpoints in any file"
        * This helps the debugger work with Rust, even if optimizations have moved your code around.

![](/images/BreakpointsEverywhere.png)

### Enable Realtime Clippy
    * Open Settings (`ctrl` + `comma`)
    * Search for "cargo check"
    * Change "Rust Analyzer > Check Command" to "clippy"
        * By default, `cargo check` provides a very quick "did it compile?" check for your code.
        * Rust has a great linter called "Clippy" built in.
        * This helps reduce errors as you work, and often provides quick refactors into idiomatic Rust.

![](/images/RustAnalyzerClippy.png)
