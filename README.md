# barh

*barh* is a simple horizontal bar chart generator inspired by
[Google Bar Chart](https://developers.google.com/chart/interactive/docs/gallery/barchart).

The main goal is a dynamic layout. No need to set chart size - it will be calculated automatically
using specified or system font. Which is lead us to no text overlapping or cropping.

**The project is still in an alpha state.**

### Build

You need the latest stable [Rust](https://www.rust-lang.org/en-US/downloads.html) compiler.

```bash
cargo build
```

### Usage

```bash
barh config.json output.svg
```

### Examples

Minimal config:
```json
{
    "items": [
        {
            "name": "Item 1",
            "value": 20
        },
        {
            "name": "Item 2",
            "value": 50
        },
        {
            "name": "Item 3",
            "value": 80
        }
    ]
}
```

Result:

![Alt text](https://cdn.rawgit.com/RazrFalcon/barh/master/examples/minimal.svg)

Complex config:
```json
{
    "title": "Complex example",
    "items_font": {
        "family": "Arial",
        "size": 11
    },
    "hor_axis": {
        "title": "Ratio",
        "width": 500,
        "max_value": 100,
        "suffix": "%"
    },
    "items": [
        {
            "name": "Item 1",
            "value": 14.4
        },
        {
            "name": "Item 2",
            "value": 55.5,
            "color": "#e34234"
        },
        {
            "name": "Item 3",
            "value": 89.1
        }
    ]
}
```

Result:

![Alt text](https://cdn.rawgit.com/RazrFalcon/barh/master/examples/complex.svg)

and with debug info:

![Alt text](https://cdn.rawgit.com/RazrFalcon/barh/master/examples/complex_with_debug.svg)

and with Noto font:

![Alt text](https://cdn.rawgit.com/RazrFalcon/barh/master/examples/complex_noto.svg)

Here we can see that bbox detection is very poor. Arial font is the best font for now.

### Limitations
 - Linux only for now since it depend on *fontconfig*.
 - Fonts. Text rendering is a pain. Text rendering in SVG is an even greater pain.
   To properly arrange items inside chart layout we need to calculate exact text bounding boxes.
   But doing it right before actual rendering is basically impossible since all OS's render fonts
   differently. Also, your machine can lack selected font or it's variants (like bold or italic).
   So we can only assume text bbox, which can lead to some rendering bugs, which can't be fixed.
 - Only simple fonts are supported. Ligatures and other staff is not supported
   (it's rather [rusttype](https://github.com/dylanede/rusttype) limitation than *barh*).
 - It's not scientific. Absolute precise of the data representation is not a goal.
 - Negative values are not supported.

### Roadmap

V0.1

 - [ ] Text bounding box detection is garbage. Write new one.
 - [ ] Windows and macOS support.
 - [ ] Make annotations optional.
 - [ ] Custom annotations.
 - [ ] Bold and cursive font detection.
 - [ ] Custom background color.
 - [ ] Custom ticks count.
 - [ ] Custom stretch value.
 - [ ] Support many values per item.

### License

*barh* is licensed under the **MIT**.
