// Topic: Implementing functionality with the impl keyword

/**
    Program requirements:
    * Print the characteristics of a shipping box
    * Must include dimensions, weight, and color

    Notes:
    * Use s struct to encapsulate the box characteristics
    * Use an enum for the box color
    * Implement functionality on the box struct to create a new box
    * Implement functionality on the box struct to print the characteristics
**/

// enum box color 
enum Color {
    Blue,
    Black,
    Brown,
}

// impl color display
impl Color {
    fn display_color(&self) {
        match self {
            Color::Blue => println!("Blue"),
            Color::Black => println!("Black"),
            Color::Brown => println!("Brown"),
        }
    }
}

// struct dimensions of box
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

// display dimensions of box
impl Dimensions {
    fn display_dimensions(&self) {
        println!("{} x {} x {}", self.width, self.height, self.depth);
    }
}

// struct characteristics of box
struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions
}

// create & display new box
impl ShippingBox {
    fn create_box(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            color,
            weight,
            dimensions
        }
    }

    fn display_characteristics(&self) {
        self.color.display_color();
        self.dimensions.display_dimensions();
        println!("Weight: {:?}", self.weight);
    }
}

fn main() {
    let small_box_dimensions = Dimensions {
        width: 1.75,
        height: 2.50,
        depth: 3.10
    };
    
    let small_blue_box = ShippingBox::create_box(5.15, Color::Blue, small_box_dimensions);
    small_blue_box.display_characteristics();
}
