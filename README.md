# lerpable

![Build Status](https://github.com/jessstringham/lerpable/actions/workflows/rust.yml/badge.svg)
![status alpha](https://img.shields.io/badge/status-alpha-red)
[![crates.io](https://img.shields.io/crates/v/lerpable.svg)](https://crates.io/crates/lerpable)

Linearly interpolate algebraic data types!

This is a derive macro that implements the trait "Lerpable", which has a function `lerpify` (apologies for the name, trying not to overwrite other names).

It used to be part of the `murrelet` package, but I extracted it to be standalone.

Using the defaults is a naive interpolation:

- it iterates down and takes all of the numbers, converts to floats, interpolates, and converts back.
- for other types, it switches at the halfway point.

You can also override it if you need a custom interpolation!

By default, `t` is a `f64`. If you are lerping between two `f64`'s `a` and `b`, it'll return `a + (b - a) * pct`.
Now let's say you are using `glam`'s `Vec2`, which has two numbers. You can implement lerpable for it, which you probably want to do by lerping the individual components

```rust
impl Lerpable for Vec2 {
    fn lerpify(&self, other: &Vec2, t: &IsLerpifyMethod) {
        vec2(
            self.x.lerpfiy(&other.x.lerpify, t),
            self.y.lerpfiy(&other.y.lerpify, t),
        )
    }
}
```

and that means interpolating between the two will trace the point traveling from `a` to `b`.

## Doing a transition other than linear

A simple Lerpable should still be all you need! Change the rate of the 'pct' you're sending in to get easing in and out.

## Skipping a type

If you want to jump straight from the start value to the end value for some field, you can give it the attribute:

```rust
#[derive(Debug, Clone, Lerpable)]
pub struct MySpecialPoint {
    #[lerpable(method="skip")]
    pub something: f32,
}
```

## I need to use someone else's data types in my lerpable data type

In general, you can slip in a custom lerp function

```rust
#[derive(Debug, Clone, Lerpable)]
pub struct MySpecialPoint {
    #[lerpable(func = "lerpify_vec2")]
    pub points: Vec2,
}
```

with the function signature

```rust
pub fn lerpify_vec2<T: lerpable::IsLerpingMethod>(this: &Vec2, other: &Vec2, pct: &T)
```

And then do your favorite newtype-wrapper or whatever.

## What is `IsLerpifyMethod`.

I left this as a trait, so you could also feed through a different function to choose between the values, e.g. for a genetic algorithms combining step. to be honest, I'm not sure what this API should look like so I just gave it all the controls I had handy:

- `with_lerp_pct()`: if this is passed part-way through an nested struct, this is called to hand it the pct. (It assumes you'll clone and return a new object, but yell if that's annoying.)
- `has_lerp_stepped`: for choosing between non-number types
- `lerp_pct`: return that 0 to 1 value
- `partial_lerp_pct`: for we're combining iterators, this is the version of the pct that is sent to the structs `lerp_partial` function if we're part-way through.
