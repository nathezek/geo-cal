use std::io::{self, Write};

fn main() {
    println!(
        r#"
 ██████╗ ███████╗ ██████╗     ██╗ ██████╗ █████╗ ██╗     
██╔════╝ ██╔════╝██╔═══██╗   ██╔╝██╔════╝██╔══██╗██║     
██║  ███╗█████╗  ██║   ██║  ██╔╝ ██║     ███████║██║     
██║   ██║██╔══╝  ██║   ██║ ██╔╝  ██║     ██╔══██║██║     
╚██████╔╝███████╗╚██████╔╝██╔╝   ╚██████╗██║  ██║███████╗
 ╚═════╝ ╚══════╝ ╚═════╝ ╚═╝     ╚═════╝╚═╝  ╚═╝╚══════╝
    "#
    );

    // ---------------------------- Main loop ------------------------------- //
    'main_menu: loop {
        println!("\n\n--------- Main Menu -----------");
        println!("1. Distance calculation");
        println!("2. Distance of Line Segment");
        println!("q. Quit program");

        print!("Enter your choice (1, 2, 3, 4 or q): ");
        io::stdout().flush().expect("Failed to flush");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice!");

        let choice = choice.trim();

        match choice {
            "q" | "quit" => {
                println!("Exited the program.");
                break 'main_menu;
            }
            "1" => {
                println!("\n\nEntering distance b/n two points. Provide the coordinates.");
                'compute_distance_loop: loop {
                    // coordinates for P:
                    println!("\nFirst coordinates:");
                    let x1 = get_input("Enter x1: ");
                    let y1 = get_input("Enter y1: ");
                    // coordinates for Q:
                    println!("\nSecond coordinates:");
                    let x2 = get_input("Enter x2: ");
                    let y2 = get_input("Enter y2: ");

                    if let (Some(x1), Some(y1), Some(x2), Some(y2)) = (x1, y1, x2, y2) {
                        println!("\nThe result is D = {}", compute_distance(x1, y1, x2, y2));

                        break 'compute_distance_loop;
                    } else {
                        println!("Please enter a valid value");
                    }
                }
            }
            _ => {
                println!("Please enter a valid number");
                continue;
            }
        }
    }
}

// ============================================= COMPUTATION FUNCATIONS ============================= //

fn compute_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    println!("\nFormula -> D = sqrt(x2 - x1)^2 + (y2 - y1)2)");
    println!(
        "\nStep 1. Plug it in -> D = sqrt ({} - {})^2 + ({} + {})^2)",
        x2, x1, y2, y1
    );
    let x = x2 - x1;
    let y = y2 - y1;
    println!("\nStep 2. We now have -> D = sqrt({}^2 + {}^2)", x, y);

    let d_in_sqrt = x.powi(2) + y.powi(2);
    println!("\nStep 3. Evaluate the squares -> D = sqrt({})", d_in_sqrt);

    d_in_sqrt.sqrt()
}

// ============================================= HELPER FUNCTIONS ===================================== //

fn get_input(prompt: &str) -> Option<f64> {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush prompt");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let input = input.trim();

    if input == "q" || input == "quit" {
        return None;
    }

    match input.parse::<f64>() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Not valid input, please enter a valid number or 'q' to quit.");
            None
        }
    }
}
