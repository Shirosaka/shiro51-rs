extern crate bit_field;

// byte wrappers representing various address types
pub mod addr;
// the brain
pub mod cpu;
// the signals of the brain
pub mod instructions;
// the memory of the brain
pub mod registers;

pub mod emi;
// a websocket server for sending information between controllers, kind of like connecting controllers through a port
pub mod ws;
