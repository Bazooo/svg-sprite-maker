![Svg Sprite Maker Icon](./src-tauri/icons/Square150x150Logo.png)

# SVG Sprite Maker

SVG Sprite Maker is a tool to easily **create, view and update** SVG sprite files

## What is an SVG Sprite?

An SVG Sprite file is a collection of symbols/icons with an id that can be referenced in HTML

```html
<svg>
    <use href="./path/to/sprite.svg#icon_id"></use>
</svg>
```

## ✨ Features

* Drag and drop SVG sprite or icon
* View and edit SVG sprite
* Add and remove icons
* Edit icon attributes with real-time preview
* Or edit icon in isolation in your editor of choice
* Check for transparency or/and alignment of icons
* 100% offline

## Tech

* [Tauri](https://tauri.app/) for desktop application
* [Svelte](https://svelte.dev/) & [Typescript](https://www.typescriptlang.org/) for the frontend
* [Rust](https://www.rust-lang.org/) for the backend

## Licensing

This project is licensed under the Apache License, Version 2.0. [License](./LICENSE)
