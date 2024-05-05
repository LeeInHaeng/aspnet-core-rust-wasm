cargo install package
================================
`cargo install wasm-pack`

`cargo install wasm-bindgen-cli`

`cargo install basic-http-server` (?? 아마 불필요)

build
================================
`cargo new --lib hello-wasm`

`dependency setting : Cargo.toml`

Cargo.toml 이 있는 디렉토리로 이동 후

cargo build --target wasm32-unknown-unknown --release

"wasm" 이라는 폴더 생성 후

`wasm-bindgen target/wasm32-unknown-unknown/release/hello_wasm.wasm --out-dir wasm --target web`
- hello_wasm.wasm 부분만 변경해서 사용 ( lib 으로 만들때 명칭으로 )

wasm 폴더에 나온 4개의 결과물 ex (hello_wasm.d.ts , hello_wasm.js , hello_wasm_bg.wasm , hello_wasm_bg.wasm.d.ts) 을

asp.net core 프로젝트의 적절한 wwwroot 경로로 이동

usage
================================
사용은 HelloWasmController 의 Index.cshtml