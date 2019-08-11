use std::env;
use fantoccini::error::CmdError;
use fantoccini::{Client, Locator};
use futures::future::Future;
use tokio;

fn main() {
   let client = fantoccini::Client::new("http://localhost:4444");
   //let args: Vec<String> = env::args().collect();
   //let entry_code = &args[1];
   //let date = &args[2];
   //let time = &args[3];
   //let amount = &args[4];

   let feedback_form = client
       .map_err(|error| unimplemented!("failed to connect to WebDriver: {:?}", error))
       .and_then(|client| client.goto("https://feedback.nandos.co.uk"))
       .and_then(|mut c| {
           c.find(Locator::Css("#NextButton"))
       })
       .and_then(|e| e.click())
       .and_then(|mut c| {
           c.find(Locator::Css("#InputStoreID"))
       })
       .map(|_| ())
       .map_err(|error| panic!("a WebDriver command failed: {:?}", error));

    tokio::run(feedback_form)
}
