build:
	clear
	RUSTFLAGS=--cfg=web_sys_unstable_apis wasm-pack build ./app --target web --out-name app --out-dir ../public/wasm

run:
	miniserve ./public --index index.html

clean:
	rm -rdf public/wasm