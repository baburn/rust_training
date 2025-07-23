mod airline;
mod seat;

use airline::{Airline, BookingError};

fn main() {
    let mut airline = Airline::new(5);

    println!("\nInitial Seat Status:");
    airline.display();

    println!("\nBooking Seat 2 for Alice:");
    match airline.book_seat(2, "Alice".to_string()) {
        Ok(_) => println!("Seat 2 booked successfully."),
        Err(e) => println!("Booking failed: {:?}", e),
    }

    println!("\nTrying to double book Seat 2:");
    if let Err(e) = airline.book_seat(2, "Bob".to_string()) {
        println!("Expected failure: {:?}", e);
    }

    println!("\nBooking Seat 10 (out of bounds):");
    if let Err(e) = airline.book_seat(10, "Charlie".to_string()) {
        println!("Expected failure: {:?}", e);
    }

    println!("\nCancelling Seat 2:");
    match airline.cancel_seat(2) {
        Ok(_) => println!("Seat 2 cancelled."),
        Err(e) => println!("Cancel failed: {:?}", e),
    }

    println!("\nFinal Seat Status:");
    airline.display();
}
