use std::error::Error;
use wayland_client::{Connection, Dispatch, QueueHandle};


fn main ()
{
    println!("You can find the suitable example # <number> here from the wayland book")
}

#[test]
fn connect_to_display()
{
    // Connect to the Wayland environment
    let connection = Connection::connect_to_env().expect("Connection Failed");

    //printout the display infor using display method. --> 
    // https://docs.rs/wayland-client/latest/wayland_client/struct.Connection.html#method.display
    eprintln!("{:?}",connection.display());
}

#[test]
fn test2()
{

}