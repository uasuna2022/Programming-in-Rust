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

fn mul_vals(values: &[NumberWithUnit]) -> NumberWithUnit {
    if values.is_empty() {
        return NumberWithUnit::unitless(1.0);
    }

    let mut result = values[0].clone();
    for item in &values[1..] {
        result.mul_in_place(item);
    }

    result
}

fn mul_vals_vec(values: Vec<NumberWithUnit>) -> NumberWithUnit {
    if values.is_empty() {
        return NumberWithUnit::unitless(1.0);
    }

    let mut result = values[0].clone();
    for item in &values[1..] {
        result.mul_in_place(item);
    }

    result
}

fn main() {
    let number1 = NumberWithUnit::with_unit(5.0, String::from("kg"));
    let number2 = NumberWithUnit::unitless(37.1);
    let number3 = NumberWithUnit::with_unit_from(number2.clone(), 42.0);

    println!("number1: {:?}", number1);
    println!("number2: {:?}", number2);
    println!("number3: {:?}", number3);

    let distance = NumberWithUnit::with_unit(100.0, String::from("m"));
    let time = NumberWithUnit::with_unit(9.58, String::from("s"));
    let speed = distance.clone().div(time.clone());

    println!("speed: {:?}", speed);

    let speed2 = NumberWithUnit::with_unit(200.0, String::from("m"))
        .clone().div(time.clone());

    println!("speed2: {:?}", speed2);
    let mut mut_var = NumberWithUnit::with_unit(137.11, String::from("m"));
    mut_var.div_in_place(&time);
    println!("mut_var: {:?}", mut_var);

    let values = [
        NumberWithUnit::with_unit(2.0, String::from("m")),
        NumberWithUnit::with_unit(3.0, String::from("s")),
        NumberWithUnit::with_unit(4.0, String::from("kg"))
    ];

    let values_vec = vec![
        NumberWithUnit::with_unit(25.0, String::from("m")),
        NumberWithUnit::with_unit(30.0, String::from("s")),
        NumberWithUnit::with_unit(41.0, String::from("kg"))
    ];

    let product1 = mul_vals(&values);
    let product2 = mul_vals(&values);

    let product_vec_1 = mul_vals_vec(values_vec.clone());
    let product_vec_2 = mul_vals_vec(values_vec);

    println!("product1: {:?}", product1);
    println!("product2: {:?}", product2);
    println!("product_vec_1: {:?}", product_vec_1);
    println!("product_vec_2: {:?}", product_vec_2);

}
