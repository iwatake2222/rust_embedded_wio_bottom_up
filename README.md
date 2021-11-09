# Embedded Rust with Wio Terminal in a bottom-up approach
# Environment
- Rust
    - https://rustup.rs/
    - Linux
        ```
        sudo apt install gcc
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        ```
    - Windows
        - Visual Studio 2019
            - Install C++ Build Tools
        - https://win.rustup.rs/x86_64
- Cross build tool, binary tool
    ```
    rustup target add thumbv7em-none-eabihf
    cargo install cargo-generate
    cargo install hf2-cli
    cargo install cargo-hf2
    ```
- Others
    - Serial terminal
    - VSCode + rust-analyzer

# Reference
基礎から学ぶ 組込みRust (中林 智之、井田 健太)
