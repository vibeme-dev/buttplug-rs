mod utils;

use buttplug::{
    client::{ButtplugClient, ButtplugClientEvent},
    connector::ButtplugInProcessClientConnector,
    test::TestDevice,
    util::async_manager,
};
use futures::StreamExt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert(
        "Setting up the client! Run this with RUST_LOG if you'd like to see library log messages.",
    );
    let connector = ButtplugInProcessClientConnector::new("Example Server", 0);
    /*
    let (client, _) = ButtplugClient::connect("Example Client", connector)
        .await
        .unwrap();
    alert("Is the client connected? {}", client.connected());
    alert("Exiting example");
    alert("Hello, buttplug-wasm!");
    */
}

#[wasm_bindgen]
pub async fn example() {

    wasm_logger::init(wasm_logger::Config::default());
    let mut connector = ButtplugInProcessClientConnector::new("Example Server", 0);
    let (_, test_device_impl_creator) =
      TestDevice::new_bluetoothle_test_device_impl_creator("Massage Demo");
    let devices = connector.server_ref().add_test_comm_manager();
    devices.lock().await.push(test_device_impl_creator);
    let (client, mut event_stream) = ButtplugClient::connect("Example Client", connector)
      .await
      .unwrap();
    if let Err(err) = client.start_scanning().await {
      println!("Client errored when starting scan! {}", err);
      return;
    }

    async_manager::spawn(async move {
      loop {
        match event_stream.next().await.unwrap() {
          ButtplugClientEvent::DeviceAdded(device) => {
            println!("We got a device: {}", device.name);
          }
          ButtplugClientEvent::ServerDisconnect => {
            println!("Server disconnected!");
          }
          _ => {
            println!("Got some other kind of event we don't care about");
          }
        }
      }
    }).unwrap();

    println!("Hit enter to continue...");
    let mut line = String::new();
    //io::stdin().read_line(&mut line).await.unwrap();

      println!("Devices currently connected:");
      for dev in client.devices() {
        println!("- {}", dev.name);
      }
    // And now we're done!
    println!("Exiting example");
    
}
