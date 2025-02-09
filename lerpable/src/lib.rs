pub use lerpable_derive::Lerpable;

pub fn step<T: Clone, LerpMethod>(this: &T, other: &T, pct: &LerpMethod) -> T
where
    LerpMethod: IsLerpingMethod,
{
    if pct.has_lerp_stepped() {
        other.clone()
    } else {
        this.clone()
    }
}

pub fn lerp<T, LerpMethod>(start: T, end: T, pct: &LerpMethod) -> T
where
    T: std::ops::Mul<f64, Output = T> + std::ops::Add<Output = T>,
    f64: std::ops::Mul<T, Output = T>,
    LerpMethod: IsLerpingMethod,
{
    let pct = pct.lerp_pct();
    (1.0 - pct) * start + pct * end
}

pub fn combine_vecs<T, LerpMethod>(this: &[T], other: &[T], pct: &LerpMethod) -> Vec<T>
where
    T: Clone + Lerpable,
    LerpMethod: IsLerpingMethod,
{
    let mut v = vec![];
    // figure out how many to show
    let this_len = this.len();
    let other_len = other.len();
    // round is important! or can get cases where two things of the same length return a count of something less!
    // I'm paranoid so also doing a special check for that case..
    let count = if this_len == other_len {
        this_len
    } else {
        lerp(this_len as f64, other_len as f64, pct).round() as usize
    };
    for i in 0..count {
        let result = match (i >= this_len, i >= other_len) {
            (true, true) => unreachable!(),
            (true, false) => {
                let emerge_pct = pct.partial_lerp_pct(i, count);
                other[i].lerp_partial(emerge_pct)
            }
            (false, true) => {
                let emerge_pct = pct.partial_lerp_pct(i, count);
                this[i].lerp_partial(emerge_pct)
            }
            (false, false) => this[i].lerpify(&other[i], pct),
        };
        v.push(result);
    }
    v
}

// these are the _methods_ used to lerp. Usually it'll just be a floating point representing
// 0 to 1. But also
//  - it could be outside of 0 and 1, yolo
//  - you could implement a newtype/etc with other ways of combining, like more random ways

pub trait IsLerpingMethod: Clone {
    fn has_lerp_stepped(&self) -> bool;

    fn partial_lerp_pct(&self, i: usize, total: usize) -> f64;

    fn lerp_pct(&self) -> f64;

    fn with_lerp_pct(&self, pct: f64) -> Self; // when introducing a new method, this will always be called first
}

impl IsLerpingMethod for f64 {
    fn has_lerp_stepped(&self) -> bool {
        *self > 0.5
    }

    fn lerp_pct(&self) -> f64 {
        *self
    }

    fn partial_lerp_pct(&self, i: usize, total: usize) -> f64 {
        // (self - i / total) * total
        // or (self * total).fract()
        *self * total as f64 - i as f64
    }

    fn with_lerp_pct(&self, pct: f64) -> Self {
        pct
    }
}

pub trait Lerpable: Sized + Clone {
    fn lerpify<T: IsLerpingMethod>(&self, other: &Self, pct: &T) -> Self;

    // by default, they just pop into existance, but you can implement this to make it fade in.
    // `pct`` will be scaled using the method's `partial_pct`
    fn lerp_partial<T: IsLerpingMethod>(&self, _pct: T) -> Self {
        self.clone()
    }
}


macro_rules! impl_lerpable {
    ($t:ty) => {
        impl Lerpable for $t {
            fn lerpify<T: IsLerpingMethod>(&self, other: &Self, pct: &T) -> Self {
                lerp(*self as f64, *other as f64, pct) as $t
            }
        }
    };
}


impl_lerpable!(usize);
impl_lerpable!(u8);
impl_lerpable!(u16);
impl_lerpable!(u64);
impl_lerpable!(i32);
impl_lerpable!(i64);
impl_lerpable!(f32);
impl_lerpable!(f64);