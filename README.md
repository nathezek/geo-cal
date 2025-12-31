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
--------- Main Menu -----------
1. Distance calculation
2. Distance of Line Segment
q. Quit program
Enter your choice (1, 2, or q): 1

Entering distance between two points. Provide the coordinates.

First coordinates:
Enter x1: 3
Enter y1: 4

Second coordinates:
Enter x2: 7
Enter y2: 1

The result is D = 5.0
```
