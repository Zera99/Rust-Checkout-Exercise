use std::io;
use std::io::Write;
const TAX_VALUE: f64 = 5.5;

fn main() {
    let mut vector_of_items: Vec<(u32, f32)> = Vec::new();
    let mut input_str = String::new();

    println!(
        "Welcome to the self-checkout system!.. Please provide the information requested below..."
    );

    handle_input(&mut input_str, &mut vector_of_items);
    display_array(&vector_of_items);
    let subtotal: f64 = process_subtotal(vector_of_items);
    let tax_total: f64 = calculate_tax(subtotal);
    let checkout_value = process_checkout(subtotal, tax_total);

    display_checkout(subtotal, tax_total, checkout_value);
}

fn handle_input(mut input_string: &mut String, vector: &mut Vec<(u32, f32)>) {
    print!("How many items are you going to take in this purchase? ");
    io::stdout().flush().unwrap();
    input_string.clear();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");

    let item_amount: usize = match input_string.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("{}", e);
            3
        }
    };

    for index in 0..item_amount {
        print!(
            "Please input how many of Item {} you're getting: ",
            index + 1
        );
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

        print!("Please input the price of Item {}: ", index + 1);
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

        vector.push((item_amount, item_price));
    }
}

fn process_subtotal(vector: Vec<(u32, f32)>) -> f64 {
    let mut subtotal: f64 = 0.0;

    subtotal += vector
        .iter()
        .map(|(a, b)| f64::from(*a as f32 * b))
        .sum::<f64>();

    return subtotal;
}

fn calculate_tax(subtotal_value: f64) -> f64 {
    return subtotal_value * TAX_VALUE / 100 as f64;
}

fn process_checkout(subtotal: f64, tax_total: f64) -> f64 {
    return subtotal + tax_total;
}

fn display_array(vector: &Vec<(u32, f32)>) {
    for (index, elem) in vector.iter().enumerate() {
        println!(
            "Item {} costs $ {:.2}, you're taking {} units ===> Total of: $ {:.2}",
            index + 1,
            elem.1,
            elem.0,
            elem.0 as f32 * elem.1
        );
    }
}

fn display_checkout(subtotal: f64, tax_total: f64, checkout_total: f64) {
    println!("\nThe subtotal is: $ {:.2}", subtotal);
    println!(
        "Tax is: $ {:.2}. Applied to your purchase it's: $ {:.2}",
        TAX_VALUE, tax_total
    );
    println!(
        "\n-- With these values, the total to pay is: $ {:.2} --\n\n",
        checkout_total
    );
}
