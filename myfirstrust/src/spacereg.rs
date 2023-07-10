pub fn helloworld() {
    let name = "Yarisingu Tarun";
    let age = 20;
    let gender = "Male";
    let weight = 82.5;
    let _is_indian = true;

    if age > 18 {
        println!("{} you are eligible for space flight", name);
        spaceflight(weight, gender);
    } else {
        println!("You are not eligible for space flight. Sorry, your data was saved.");
    }
}

fn spaceflight(weight: f64, _gender: &str) {
    if weight > 50.0 {
        let ticket_price = weight * 10.0;
        println!("Your ticket price is {}", ticket_price);
        println!("Ready for space flight within a few days");
    }
}
