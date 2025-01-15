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
5. 使用下述命令编译
```
typst-hugo compile ./main.typ --html-dir ../content/posts/ --asset-dir ../static/typst-ts --path-to-root /typst-ts/
```
6. `hugo`

## 黑暗模式
目前黑暗模式只适配了MeME/Blowfish 主题，如果想要适配其他主题，则需手动实现`window.getTypstTheme` 函数， 并且在切换黑暗/白天模式时调用 `window.typstChangeTheme()` 函数

## 测试过的主题
- [x] MeME
- [x] Blowfish

## Acknowledgments
很多的代码是从 shiroa 和 typst.ts 抄的.
感谢 @Myriad-Dreamin.
- [typst.ts](https://github.com/Myriad-Dreamin/typst.ts)
- [shiroa](https://github.com/Myriad-Dreamin/shiroa)