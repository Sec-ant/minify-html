# minify-html-wasm

A wasm wrapper of the rust crate [minify-html](https://crates.io/crates/minify-html).

## Install

```bash
npm i minify-html-wasm
```

## Usage

```ts
import init, { minify } from "minify-html-wasm";

const encoder = new TextEncoder();
const decoder = new TextDecoder();

await init("https://fastly.jsdelivr.net/npm/minify-html-wasm@0.1.1/dist/web/index_bg.wasm");

const minified = decoder.decode(
  minify(encoder.encode("<p>  Hello, world!  </p>"), {
    keep_spaces_between_attributes: true,
    keep_comments: true,
  })
);
```
