# lerpable

Linearly interpolate algebraic data types!

This is a derive macro that implements the trait "Lerpable", which has a function `lerpify` (apologies for the name, trying not to overwrite other names).

It used to be part of the `murrelet` package, but I extracted it to be standalone.

Using the defaults is useful for a first pass at transitioning animations. You can then implement the traits if you need a custom interpolation. For example, if you have something that "draws a polygon with X sides", by default it might awkwardly add in more sides at the same location.

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


# FAQ

## What if I want to do something other than linear interpolate?

Lerpable does this for you! Change the rate of the 'pct' you're sending in.

## Does it clone?

yes

## What is `IsLerpifyMethod`.

I left this as a trait, so you could also feed through a different function to choose between the values, e.g. for a genetic algorithms combining step.