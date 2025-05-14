use std::error::Error;
use wayland_client::{Connection, Dispatch, EventQueue, Proxy, QueueHandle, globals};
use wayland_server::{Display, DisplayHandle};
// use wayland_client::Proxy
fn main() {
    println!("You can find the suitable example # <number> here from the wayland book")
    // link for the wayland-book --->  https://wayland-book.com/wayland-display/creation.html
}

#[test]
// Example #1
fn connect_to_display() {
    // Connect to the Wayland environment
    let connection = Connection::connect_to_env().expect("Connection Failed");

    //printout the display infor using display method. -->
    // https://docs.rs/wayland-client/latest/wayland_client/struct.Connection.html#method.display
    eprintln!("{:?}", connection.display());

    // In my case $XDG_RUNTIME/$WAYLAND_DISPALY = /run/user/1000/wayland-1
    // but it is mostly wayland-0
}

#[test]
// Example #2 --> Not Complete :::
fn create_wayland_server_print_socket_name() {
    #[derive(Debug)]
    struct ServerState {}
    // let state= ServerState{};

    //Rather than creating an eventLoop we simply want to print socket_name here
    //to use these you need to have wayland-server lib
    let mut display: Display<ServerState> =
        Display::new().expect("Failed to Create a display Server");

    let handle = &display.handle();

    let flush_clients = display.flush_clients();

    println!("Display Variable --> {:?}", &display);
    println!("Handle --> {:?}", &handle);
    println!("Flush Clients --> {:?}", &flush_clients);
}

#[test]

fn registry_listener_test() {
   //Globals --> Helper For an application 
   //https://docs.rs/wayland-client/latest/wayland_client/globals/index.html

   // Implement the Dispatch Trait for Your Struct (State Here) --> Make a connection to env --> use registry_queue_init to get globals and the event_queue

    use wayland_client::globals::GlobalList;
    use wayland_client::{
        Connection, Dispatch, QueueHandle,
        globals::{Global, GlobalListContents, registry_queue_init},
        protocol::{wl_compositor, wl_registry},
    };

    // struct RegistryHandler;

    struct State;

    // the following can be found at https://docs.rs/wayland-client/latest/wayland_client/globals/index.html

    // You need to provide a Dispatch<WlRegistry, GlobalListContents> impl for your app
    impl wayland_client::Dispatch<wl_registry::WlRegistry, GlobalListContents> for State {
        fn event(
            state: &mut State,
            proxy: &wl_registry::WlRegistry,
            event: wl_registry::Event,
            // This mutex contains an up-to-date list of the currently known globals
            // including the one that was just added or destroyed
            data: &GlobalListContents,
            conn: &Connection,
            qhandle: &QueueHandle<State>,
        ) {
            match event {
                wl_registry::Event::Global {
                    name,
                    interface,
                    version,
                } => {
                    println!(
                        "interface: {} , version: {} , name: {}",
                        interface, version, name
                    )
                }
                _ => {
                    println!("The _ arm was envoked")
                }
            }
        }
    }

    //now lets create a connection

    let connection = Connection::connect_to_env().expect("Failed connection");

    // { // we don't need a event_queue to extract globals ---> https://docs.rs/wayland-client/latest/wayland_client/globals/index.html 

    // let mut event_queue = connection.new_event_queue::<State>();
    // let qh = event_queue.handle();
    // this does not exist :)
    // let globals= GlobalList::new(&connection,&qh);

    // }

    // registry_queue_init --> Fetches globals and event_queue

    let (globals, queue) = registry_queue_init::<State>(&connection).unwrap();

    println!("All globals: {:?}\n", globals.contents());
    println!("Event Queue: {:?}\n", queue);

}
