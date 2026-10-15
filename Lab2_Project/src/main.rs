struct NumberWithUnit {
    unit: String,
    value: f64,
}

impl NumberWithUnit {
    fn unitless(value: f64) -> Self {
        Self {
            unit: String::new(),
            value,
        }
    }

    fn with_unit(value: f64, unit: String) -> Self {
        Self {
            unit,
            value,
        }
    }

    fn with_unit_from(other: Self, value: f64) -> Self {
        Self {
            unit: other.unit.clone(),
            value,
        }
    }
}


fn main() {
    println!("Hello, world!");
}
