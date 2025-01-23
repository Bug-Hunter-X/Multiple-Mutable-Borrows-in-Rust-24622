# Multiple Mutable Borrows in Rust

This example demonstrates a common error in Rust related to mutable borrowing.  Rust's ownership system prevents data races by disallowing multiple mutable borrows to the same variable at the same time. Attempting to do so results in a compile-time error.

## The Bug

The `bug.rs` file shows the code that causes the error. The core issue is creating two mutable references (`y` and `z`) to the same variable `x` simultaneously. 

## The Solution

The solution demonstrates how to modify the code to avoid the error. The solution involves using a different approach to manipulate the variable so that the Rust borrow checker can ensure no data race occurs.

## How to Run

1. Save the code in `bug.rs` and `bugSolution.rs`.
2. Compile using `rustc bug.rs` and `rustc bugSolution.rs`.
3. Run the compiled executable. 