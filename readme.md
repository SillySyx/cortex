<div align="center">
    <h1><code>Cortex</code></h1>
    <strong>An offline password manager and knowledgebase application.</strong>
</div>


# Features
* Password manager
* Knowledgebase
* Data encryption
* Webassembly
* Offline progressive web application


# Requirements
* Rust

Setup
```
cargo install wasm-pack
cargo +nightly install miniserve
```


# Todo
* Setup automatic github pages deployment, trigger on tags
* Notification when password has been copied
* Remove password from clipboard after 10sec
* Automatic passphrase generation when adding passwords
* Import/Export passwords
* Knowledgebase


# Resources
* https://github.com/yewstack/yew
* https://rustwasm.github.io/docs/wasm-bindgen/