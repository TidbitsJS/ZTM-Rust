struct Temperature {
    degrees_f: f64,
}

/**
    impl Temperature {
        fn show_temp(temp: Temperature) {
            println!("{:?} degrees F", temp.degrees_f);
        }
    }
**/

impl Temperature {
    fn freezing() -> Self {
        Self {
            degrees_f: 15.21
        }
    }

    fn boiling() -> Self {
        Self {
            degrees_f: 212.77
        }
    }

    fn show_temp(&self) {
        println!("{:?} degrees F", self.degrees_f);
    }
}

fn main() {
    let hot = Temperature { degrees_f: 100.21 };
    // Temperature::show_temp(hot);
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();

    let boiling = Temperature::boiling();
    boiling.show_temp();
}
