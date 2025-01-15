# typst-hugo
<a href="./README.zh_CN.md">中文</a>|
<a href="./README.md">English</a>

A simple typst to responsive html compiler for hugo.

## [Demo](https://typst-hugo-demo.pages.dev/)

## Install

```bash
cargo install --git https://github.com/wenbin-liu/typst-hugo.git --locked   
```  

## Usage
1. cd into the hugo site directory
2. `mkdir typst`
3. `cd typst`
4. `typst-hugo template`
5. compile with following command
```
typst-hugo compile ./main.typ --html-dir ../content/posts/ --asset-dir ../static/typst-ts --path-to-root /typst-ts/
```
6. `hugo`

## About Dark Mode
The dark mode is only suited for MeME/Blowfish theme. If you
want to use dark mode in your theme. You need to
implement ` window.getTypstTheme` and invoke `window.typstChangeTheme()` when you toggle dark/light change in the output html.

## Tested Theme
- [x] MeME
- [x] Blowfish

## Acknowledgments
Many code are borrowed directly fron shiroa and typst.ts. Thanks to @Myriad-Dreamin.
- [typst.ts](https://github.com/Myriad-Dreamin/typst.ts)
- [shiroa](https://github.com/Myriad-Dreamin/shiroa)
