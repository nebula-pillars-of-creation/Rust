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
        path. Restart your terminal or command prompt to ensure the changes take effect.
    </step>
    <step>
        <b>Test installation:</b> Run <code>rustc --version</code> and <code>cargo --version</code> to confirm that Rust and 
        Cargo are installed correctly.
        <img src="terminal-rust-version.png" alt="Terminal: rustc --version"/>
    </step>
</procedure>

### Understanding version management with rustup

