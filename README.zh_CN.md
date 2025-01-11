# typst-hugo
<a href="./README.zh_CN.md">中文</a>|
<a href="./README.md">English</a>

一个简单的 typst 到 html 的生成器。
支持响应式渲染。

## [Demo](https://typst-hugo-demo.pages.dev/)

## 安装

```bash
cargo install --git https://github.com/wenbin-liu/typst-hugo.git --locked   
```  

## 使用
1. 进入hugo site 文件夹
2. `mkdir typst`
3. `cd typst`
4. `typst-hugo template`
5. 使用下述命令比阿尼以
```
typst-hugo compile ./main.typ --html-dir ../content/posts/ --asset-dir ../static/typst-ts --path-to-root /typst-ts/
```
6. `hugo`