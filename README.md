# yunter
A Rust library to convert from one color-space to another.

Documentation can be found on http://docs.rs/yunter.

# Origin of the name

`yunter` is named after [Thomas Young](https://en.wikipedia.org/wiki/Thomas_Young_(scientist)) and [Richard S. Hunter](https://en.wikipedia.org/wiki/Richard_S._Hunter), who worked on [colour theory](https://en.wikipedia.org/wiki/Thomas_Young_(scientist)#Vision_and_colour_theory) and the [Hunter Lab color space](https://en.wikipedia.org/wiki/Lab_color_space#Hunter_Lab).

# Examples

```rust
extern crate yunter;

let rgb = Rgb::new(176, 57, 209);

let xyz: Xyz = rgb.into();

let lab: Lab = rgb.into();
```
