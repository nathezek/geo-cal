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
        println!("l. List all forms of line equations");
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
            "list" | "l" => {
                println!("\n\nShowing list of forms of line equations: ");
                println!(
                    r#"
                Point Slope: y - y1 = m( x - x1 )
                "#
                );
                println!(
                    r#"
                Slope Intercept: y = mx + b 
                "#
                );
                println!(
                    r#"
                                     y2 - y1
                Two point: y - y1 = --------- ( x - x1 ) 
                                     x2 - x1
                "#
                );
            }
            "1" => {
                println!("\n\nEntering distance b/n two points. Provide the coordinates.");
                'compute_distance_loop: loop {
                    // coordinates for P:
                    println!("\nFirst coordinates:");
                    let x1 = get_input("Enter x1: ");
                    if x1.is_none() {
                        break 'compute_distance_loop;
                    }
                    let y1 = get_input("Enter y1: ");
                    if y1.is_none() {
                        break 'compute_distance_loop;
                    }
                    // coordinates for Q:
                    println!("\nSecond coordinates:");
                    let x2 = get_input("Enter x2: ");
                    if x2.is_none() {
                        break 'compute_distance_loop;
                    }
                    let y2 = get_input("Enter y2: ");
                    if y2.is_none() {
                        break 'compute_distance_loop;
                    }

                    if let (Some(x1), Some(y1), Some(x2), Some(y2)) = (x1, y1, x2, y2) {
                        println!("\nThe result is D = {}", compute_distance(x1, y1, x2, y2));

                        break 'compute_distance_loop;
                    } else {
                        println!("Please enter a valid value");
                    }
                }
            }
            "2" => {
                println!(
                    "Entering distance of Line Segment, Provide the coordinates then the ratio: "
                );
                'compute_distance_of_line_segment_loop: loop {
                    // coordinates for P:
                    println!("\nFirst coordinates:");
                    let x1 = get_input("Enter x1: ");
                    if x1.is_none() {
                        break 'compute_distance_of_line_segment_loop;
                    }
                    let y1 = get_input("Enter y1: ");
                    if y1.is_none() {
                        break 'compute_distance_of_line_segment_loop;
                    }
                    // coordinates for Q:
                    println!("\nSecond coordinates:");
                    let x2 = get_input("Enter x2: ");
                    if x2.is_none() {
                        break 'compute_distance_of_line_segment_loop;
                    }
                    let y2 = get_input("Enter y2: ");
                    if y2.is_none() {
                        break 'compute_distance_of_line_segment_loop;
                    }
                    // ratio of r1 and r2:
                    println!("\nRatio:");
                    let r1 = get_input("Enter r1: ");
                    if r1.is_none() {
                        break 'compute_distance_of_line_segment_loop;
                    }
                    let r2 = get_input("Enter r2: ");
                    if r2.is_none() {
                        break 'compute_distance_of_line_segment_loop;
                    }

                    if let (Some(x1), Some(y1), Some(x2), Some(y2), Some(r1), Some(r2)) =
                        (x1, y1, x2, y2, r1, r2)
                    {
                        println!(
                            "\nThe result is R = {:?}",
                            compute_distance_of_line_segment(x1, y1, x2, y2, r1, r2)
                        );

                        break 'compute_distance_of_line_segment_loop;
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

// ============================================= COMPUTATION FUNCTIONS ============================= //

fn compute_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    println!("\nWe are given: P({},{}) & Q ({},{})", x1, y1, x2, y2);
    println!(
        r#"
                      ______________________________
    Formula -> D =  \| (x2 - x1)^2  +  (y2 - y1)^2 
        
    "#
    );
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

fn compute_distance_of_line_segment(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    r1: f64,
    r2: f64,
) -> (f64, f64) {
    println!(
        "\nWe are given P({},{}) , Q({},{}) & ratio {}:{}",
        x1, y1, x2, y2, r1, r2
    );
    println!(
        r#"

     Formula -> R =    /                                 \
                      |  x1.r2 + x2.r1  ,  y1.r2 + y2.r1  |
                      |  -------------     -------------  |
                      |    r1 + r2            r1 + r2     |
                       \                                 /
    "#
    );
    let sumation_x = (x1 * r2) + (x2 * r1);
    let sumation_y = (y1 * r2) + (y2 * r1);
    let sumation_r = r1 + r2;

    let first_part = sumation_x / sumation_r;
    let second_part = sumation_y / sumation_r;

    (first_part, second_part)
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
