enum Light {
    Bright,
    Dull
}

fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("The light is bright!"),
        Light::Dull => println!("The light is dull.")
    }
}

fn main() {
    let dull = Light::Dull;
    
    display_light(&dull);
    display_light(&dull);
}
