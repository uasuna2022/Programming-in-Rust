// 'Copy' feature can't be used, because String doesn't have 'Copy' trait implemented.
#[derive(Debug, Clone, Default)]
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

    fn add(self, other: Self) -> Self {
        if self.unit != other.unit {
            panic!("Can't add numbers with different units: {} and {}",
                self.unit, other.unit);
        }

        Self {
            unit: self.unit,
            value: self.value + other.value,
        }
    }

    fn mul(self, other: Self) -> Self {
        Self {
            unit: format!("{}*{}", self.unit, other.unit),
            value: self.value * other.value,
        }
    }

    fn div(self, other: Self) -> Self {
        Self {
            unit: format!("{}/{}", self.unit, other.unit),
            value: self.value / other.value,
        }
    }

    fn add_in_place(&mut self, other: &Self) {
        if self.unit != other.unit {
            panic!("Can't add numbers with different units: {} and {}",
                self.unit, other.unit);
        }

        self.value += other.value;
    }

    fn mul_in_place(&mut self, other: &Self) {
        self.unit = format!("{}*{}", self.unit, other.unit);
        self.value *= other.value;
    }

    fn div_in_place(&mut self, other: &Self) {
        self.unit = format!("{}/{}", self.unit, other.unit);
        self.value /= other.value;
    }
}


fn main() {
    let number1 = NumberWithUnit::unitless(42.0);
    let number2 = NumberWithUnit::with_unit(37.1, String::from("kg"));
    let number3 = NumberWithUnit::with_unit_from(number2.clone(), 50.0);

    println!("Number 1: {:?}", number1);
    println!("Number 2: {:?}", number2);
    println!("Number 3: {:?}", number3);

    let mut distance = NumberWithUnit::with_unit(100.0, String::from("m"));
    let time = NumberWithUnit::with_unit(3.5, String::from("s"));
    let speed = distance.clone().div(time.clone());

    println!("Speed: {:?}", speed);
    distance.div_in_place(&time);
    println!("Distance after division: {:?}", distance); 
}
