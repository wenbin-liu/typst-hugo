#import "@preview/shiroa:0.1.2": get-page-width, templates, is-web-target, target
#import templates: book-theme-from, heading-hash
#import "@preview/codly:1.2.0": codly-init

#let project(title: [], author: (), date: none, categories: (), tags: (), lang: "en", region: "us", body) = {
  set document(title: title, date: date, author: author)

  let raw-meta = (categories: categories, tags: tags);

  let themes = (
    light: (color-scheme: "light", main-color: rgb("#000"), dash-color: rgb("#20609f")),
    dark: (color-scheme: "dark", main-color: rgb("#c5c5c5"), dash-color: rgb("#0096cf")),
  )
  let main-font = ("Libertinus Serif")
  let code-font = ("BlexMono Nerd Font Mono", "DejaVu Sans Mono",)

  let theme-target = if target.contains("-") {
    target.split("-").at(1)
  } else {
    "light"
  }
  let (main-color: main-color, dash-color: dash-color) = themes.at(theme-target)

  let main-size = 16pt
  let heading-sizes = (26pt, 22pt, 14pt, 12pt, main-size)
  let list-indent = 0.5em

  set text(lang: lang, region: region)
  set text(font: main-font, size: main-size, fill: main-color)

  set page(width: get-page-width(), height: auto, margin: (
    // reserved beautiful top margin
    top: 20pt,
    // reserved for our heading style.
    // If you apply a different heading style, you may remove it.
    left: 20pt,
    right: 5pt,
    // Typst is setting the page's bottom to the baseline of the last line of text. So bad :(.
    bottom: 0.5em,
    // remove rest margins.
    rest: 0pt,
  ))
  set par(justify: true)

  // Set main spacing

  set enum(indent: list-indent * 0.618, body-indent: list-indent)
  set list(indent: list-indent * 0.618, body-indent: list-indent)
  set par(leading: 0.7em)
  set block(spacing: 0.7em * 1.5)

  // Set text, spacing for headings
  // Render a dash to hint headings instead of bolding it as well if it's for web.
  show heading: set text(weight: "regular")
  show heading: it => {
    let it = {
      set text(size: heading-sizes.at(it.level))
      heading-hash(it, hash-color: dash-color)
      it
    }

    block(spacing: 0.7em * 1.5 * 1.2, below: 0.7em * 1.2, it)
  }

  // link setting
  show link: set text(fill: dash-color)

  // math setting
  show math.equation: set text(weight: 400)

  show: codly-init.with()

  [#metadata(raw-meta) <typst_hugo_0xbafe783>]
  body
}