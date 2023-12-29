# Environment Setup

<show-structure for="chapter" depth="3"/>

## Installing Rust with rustup

### Step-by-step installation guide

<procedure title="Installing Rust with rustup">
    <step>
        <b>Download rustup:</b> Rust’s installation starts with rustup, the Rust toolchain installer.
        Visit the official [Rust website](https://www.rust-lang.org/learn/get-started) to download rustup for your 
        operating system.
    </step>
    <step>
        <b>Install Rust:</b>
        <tabs>
            <tab title="Unix/Linux (WSL)">
                <p>On Unix-based systems, this is usually a command like:</p>
                <code-block lang="bash">
                    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
                </code-block>
            </tab>
            <tab title="Windows">
                <p>You may install using these links:</p>
                <list>
                <li>
                    <a href="https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe">
                    Download Rustup [x32]</a>
                </li>
                <li>
                    <a href="https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe">
                    Download Rustup [x64]</a>
                </li>
                </list>
            </tab>
        </tabs>
    </step>
    <step>
        <b>Configure your path:</b> After installation, rustup will provide instructions to add Rust to your system 
        path. Restart your terminal or command prompt to ensure the changes take effect. In the Rust development
        environment, all tools are installed to the <code>~/.cargo/bin</code> directory, and this is where you will 
        find the Rust toolchain, including rustc, cargo, and rustup.
        <tabs>
          <tab title="cargo bin">
            <code-block lang="plain text" src="cargo-help.txt" include-lines="1-18" collapsible="true"/>
          </tab>
          <tab title="cargo help">
            <code-block lang="plain text" src="cargo-help.txt" include-lines="22-62" collapsible="true"/>
          </tab>
        </tabs>
    </step>
    <step>
        <b>Test installation:</b> Run <code>rustc --version</code> and <code>cargo --version</code> to confirm that Rust and 
        Cargo are installed correctly.
        <img src="terminal-rust-version.png" alt="Terminal: rustc --version"/>
    </step>
</procedure>

### Understanding version management with rustup

- **Update Toolchain:** To update Rust, use `rustup update`.
- **Manage Versions:** You can manage different versions for different projects using rustup override set followed by
  the version number.
- **Nightly Builds:** For accessing features not yet in stable Rust, you can switch to the nightly Rust channel
  with `rustup default nightly`.

<tabs>
    <tab title="rustup help">
        <code-block lang="plain text" src="rust-help.txt" validate="false" include-lines="1-57" collapsible="true"/>
    </tab>
    <tab title="rustc help">
        <code-block lang="plain text" src="rust-help.txt" validate="false" include-lines="62-103" collapsible="true"/>
    </tab>
</tabs>

## Understanding cargo Tool

### Basics of cargo

- Cargo is Rust’s build system and package manager.
- It can initialize new projects with cargo new `<project_name>`, manage dependencies, and build projects.
- Dependencies are specified in `Cargo.toml`, and `Cargo.lock` ensures consistent builds.

### Creating a new project, managing dependencies, and building your project.

<procedure>
  <step>
    <p><b>Create New Project:</b> Use <code lang="console">cargo new [project_name]</code> to create a new project.
    This creates a new directory with the project name, initializes a Git repository, and creates initial files.</p>
  </step>
  <step>
    <p><b>Manage Dependencies:</b> Add dependencies by specifying them under [dependencies] in <code>Cargo.toml</code>. 
    Cargo will handle downloading and compiling the dependencies.</p>
  </step>
  <step>
    <p><b>Build Project:</b> Run <code lang="console">cargo build</code> to compile your project. For a release build, 
    use <code lang="console">cargo build --release</code>.</p>
  </step>
</procedure>
<video src="rust-example.mp4" preview-src="cargo-example.png" width="706"/>

## Choosing an IDE or Editor with Rust Support

### Overview of popular IDEs and editors

- **[Visual Studio Code (VS Code)](https://code.visualstudio.com/Download)** with
  the [Rust extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) is a popular choice
  due to its rich features and extensive
  community support.
- **[JetBrains RustRover](https://www.jetbrains.com/rust/)** with the Rust plugin offers a robust environment for Rust
  development.
- **Sublime Text, Atom, and Vim/Neovim** are also widely used with Rust plugins or extensions.
