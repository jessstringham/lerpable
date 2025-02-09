use std::collections::HashMap;

use lerpable::{IsLerpingMethod, Lerpable};

#[derive(Debug, Clone, Lerpable)]
pub struct BasicTypes {
    s: String,
    a_number: f32,
    b_number: usize,
    something: Vec<f32>,
}

#[derive(Clone)]
struct CustomMethod {
    pct: f64,
}
impl IsLerpingMethod for CustomMethod {
    fn has_lerp_stepped(&self) -> bool {
        false
    }

    fn partial_lerp_pct(&self, _i: usize, _total: usize) -> f64 {
        0.0
    }

    fn lerp_pct(&self) -> f64 {
        0.0
    }

    fn with_lerp_pct(&self, pct: f64) -> Self {
        let mut c = self.clone();
        c.pct = pct;
        c
    }
}

fn custom_method() -> CustomMethod {
    CustomMethod { pct: 0.0 }
}

fn custom_func<T: IsLerpingMethod>(this: &f32, other: &f32, _pct: &T) -> f32 {
    *this - *other
}

#[derive(Debug, Clone, Lerpable)]
pub struct BasicTypesWithOverrides {
    #[lerpable(func = "custom_func")]
    a_number: f32,
    #[lerpable(method = "custom_method")]
    something: Vec<f32>,
    #[lerpable(method = "skip")]
    label: String,
    #[lerpable(method = "skip")]
    b: HashMap<String, String>,
}

#[derive(Debug, Clone, Lerpable)]
enum EnumTest {
    A,
    B(BasicTypesWithOverrides),
}

fn main() {
    let a = EnumTest::A;
    let b = EnumTest::B(BasicTypesWithOverrides {
        a_number: 23.0,
        something: vec![],
        label: "test".to_owned(),
        b: HashMap::new(),
    });
    a.lerpify(&b, &0.75);
}
