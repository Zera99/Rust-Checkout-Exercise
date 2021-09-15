use std::io;
use std::io::Write;

const ARRAY_SIZE: usize = 3;
const TAX_VALUE: f64 = 5.5;

fn main() {
    let mut array_of_items: [(u32, f32); ARRAY_SIZE] = [(0, 0.0); ARRAY_SIZE];
    let mut input_str = String::new();

    println!(
        "Welcome to the self-checkout system!.. Please provide the information requested below..."
    );

    handle_input(&mut input_str, &mut array_of_items);
    display_array(array_of_items);
    let subtotal: f64 = process_subtotal(array_of_items);
    let tax_total: f64 = calculate_tax(subtotal);
    let checkout_value = process_checkout(subtotal, tax_total);

    display_checkout(subtotal, tax_total, checkout_value);
}

fn handle_input(mut input_string: &mut String, array: &mut [(u32, f32); ARRAY_SIZE]) {
    for i in 0..ARRAY_SIZE {
        print!("Please input how many of Item {} you're getting: ", i + 1);
        io::stdout().flush().unwrap();
        input_string.clear();
        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");

        println!("{}", input_string);

        let item_amount: u32 = match input_string.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        print!("Please input the price of Item {}: ", i + 1);
        io::stdout().flush().unwrap();
        input_string.clear();
        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");

        println!("{}", input_string);

        let item_price: f32 = match input_string.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        array[i] = (item_amount, item_price);
    }
}

fn process_subtotal(array: [(u32, f32); ARRAY_SIZE]) -> f64 {
    let mut subtotal: f64 = 0.0;
    for i in 0..ARRAY_SIZE {
        subtotal += (array[i].0 as f32 * array[i].1) as f64;
    }

    return subtotal;
}

fn calculate_tax(subtotal_value: f64) -> f64 {
    return subtotal_value * TAX_VALUE / 100 as f64;
}

fn process_checkout(subtotal: f64, tax_total: f64) -> f64 {
    return subtotal + tax_total;
}

fn display_array(array: [(u32, f32); ARRAY_SIZE]) {
    for i in 0..ARRAY_SIZE {
        println!(
            "Item {} costs $ {:.2}, you're taking {} units ===> Total of: $ {:.2}",
            i + 1,
            array[i].1,
            array[i].0,
            array[i].0 as f32 * array[i].1
        );
    }
}

fn display_checkout(subtotal: f64, tax_total: f64, checkout_total: f64) {
    println!("The subtotal is: $ {:.2}", subtotal);
    println!(
        "Tax is: $ {:.2}. Applied to your purchase it's: $ {:.2}",
        TAX_VALUE, tax_total
    );
    println!(
        "With these values, the total to pay is: $ {:.2}",
        checkout_total
    );
}
