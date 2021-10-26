use std::io::stdin;

fn print_menu(vector: &Vec<(&str, f32)>) {
    println!("Menu:");
    println!("=======================");
    
    for n in vector.iter() {
        let food = n.0;
        let price = n.1;

        let formatted_text = 
            format!("{:<12}{:>12}", food, price,);
        println!("{}", formatted_text);

    }
}

fn ask_quantity (vector: &Vec<(&str, f32)>) -> f32 {
    let mut list: f32 = 0.0;
    
    for n in vector.iter() {
        let mut value = String::new();
        
        println!("Enter the number of {}", n.0);
        stdin()
            .read_line(&mut value)
            .expect("Please re-enter the value.");

        let value: f32 = value.trim().parse().expect("hello");

        list = list + value;
    }

    return list;
}

fn main() {

    // setting food first in a vector with a primitive string literal...
    let food: Vec<&str> = vec![
      "Soup", "Main Dish", "Dessert", "Drink"
    ];

    // then a vector with our floating-point vectors
    let prices: Vec<f32> = vec![
      3.75, 6.25, 2.00, 1.50
    ];

    // then lastly iterates through both arrays into another vector with a pair containing the food[n] and prices[n] 
    let menu: Vec<(&str, f32)> = 
    food.into_iter().zip(prices.into_iter()).collect();
    
    // first we present the menu to the end user by passing in a reference to our menu vector
    print_menu(&menu);

    // then we'll ask for the quantity of each item present in the menu, then store the returned value (which is the total in dollars)
    let ask = ask_quantity(&menu);
    println!("{:?}", ask);

    // now to finish off, we first print off the total before tax, which was the return value of ask_quantity(&menu)
    // then store the value of the total * 0.13 (tax at 13%)
    // and lastly, prints off the sum of both the total and the tax!
    
}
