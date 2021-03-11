<div align="center">
    <h1><code>Cortex</code></h1>
    <strong>The freshest, spiciest, hottest and most progressive application on the web!</strong>
</div>


# Features
* Progressive web application
* Webassembly


# Requirements
* Rust
* NodeJS


# Build
**Setup**
```
rustup target add wasm32-unknown-unknown

cd app
npm i
```

**Building wasm**
```
cd lib
wasm-pack build --target no-modules
```

**Building app**
```
cd app
npm build
```


# Release
...