## Geo / Cal

**Geo / Cal** is a simple command-line application written in Rust that calculates the distance between two points in 2D space. This application uses the Euclidean distance formula to provide accurate distance calculations.

### Features

- Calculate the distance between two points in 2D space.
- User-friendly command-line interface.
- Handles invalid input gracefully.
- Option to exit the program at any time by entering 'q' or 'quit'.

### Installation

You can install Geo / Cal using Cargo, Rust's package manager. Simply run the following command:

```bash
cargo install geo-cal
```
### Usage
```bash
geo-cal
```
### Example interaction
```code
 ██████╗ ███████╗ ██████╗     ██╗ ██████╗ █████╗ ██╗
██╔════╝ ██╔════╝██╔═══██╗   ██╔╝██╔════╝██╔══██╗██║
██║  ███╗█████╗  ██║   ██║  ██╔╝ ██║     ███████║██║
██║   ██║██╔══╝  ██║   ██║ ██╔╝  ██║     ██╔══██║██║
╚██████╔╝███████╗╚██████╔╝██╔╝   ╚██████╗██║  ██║███████╗
 ╚═════╝ ╚══════╝ ╚═════╝ ╚═╝     ╚═════╝╚═╝  ╚═╝╚══════╝



--------- Main Menu -----------
1. Distance calculation
2. Distance of Line Segment
l. List all forms of line equations
q. Quit program
Enter your choice (1, 2, 3, 4 or q): 1


Entering distance b/n two points. Provide the coordinates.

First coordinates:
Enter x₁: 1
Enter y₁: 1

Second coordinates:
Enter x₂: 4
Enter y₂: 4

We are given: P(1,1) & Q (4,4)

    Formula -> D =  √((x₂ - x₁)²  +  (y₂ - y₁)²)


Step 1. Plug it in -> D = √((4 - 1)² + (4 + 1)²)

Step 2. We now have -> D = √(3² + 3²)

Step 3. Evaluate the squares -> D = √18

The result is D = 4.242640687119285
```
