use crate::seat::Seat;

#[derive(Debug)]
pub enum BookingError {
    SeatAlreadyBooked,
    SeatDoesNotExist,
}

pub struct Airline {
    pub seats: Vec<Option<Seat>>,
}

impl Airline {
    pub fn new(total_seats: usize) -> Self {
        Airline {
            seats: vec![None; total_seats],
        }
    }

    pub fn book_seat(&mut self, index: usize, name: String) -> Result<(), BookingError> {
        match self.seats.get_mut(index) {
            Some(slot @ None) => {
                *slot = Some(Seat {
                    seat_number: index,
                    passenger_name: name,
                });
                Ok(())
            }
            Some(Some(_)) => Err(BookingError::SeatAlreadyBooked),
            None => Err(BookingError::SeatDoesNotExist),
        }
    }

    pub fn cancel_seat(&mut self, index: usize) -> Result<(), BookingError> {
        match self.seats.get_mut(index) {
            Some(Some(_)) => {
                self.seats[index] = None;
                Ok(())
            }
            Some(None) => Err(BookingError::SeatDoesNotExist),
            None => Err(BookingError::SeatDoesNotExist),
        }
    }

    pub fn display(&self) {
        for (i, seat) in self.seats.iter().enumerate() {
            match seat {
                Some(s) => println!("Seat {} -> Booked by {}", i, s.passenger_name),
                None => println!("Seat {} -> Available", i),
            }
        }
    }
}
