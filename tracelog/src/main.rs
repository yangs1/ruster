use std::{char::from_u32, fs::File, error::Error, sync::Mutex};

use tracing_subscriber::{FmtSubscriber, fmt::MakeWriter};
use tracing::{info, error, Level, span, event, info_span};

fn main() {
    let  buffer = File::create("foo.txt").unwrap();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_file(true)
        // .with_target(true)
        .with_writer(Mutex::new(buffer))
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default error");
    let number_of_trams: i32 = 3;
    info!("we`ve got {} team!", number_of_trams);

   // records an event outside of any span context:
event!(Level::INFO, "something happened");


let span = info_span!("my_great_span");

{
    let _enter = span.enter();

    // this event occurs inside the span.
    info!("i'm in the span!");

    // exiting the scope drops the guard, exiting the span.
}

// this event is not inside the span.
info!("i'm outside the span!");


let span = span!(Level::INFO, "my_span");

let  id = span.id().unwrap().into_u64();
info!("id: {}", id);
let _guard = span.enter();

// records an event within "my_span".
event!(Level::INFO, "something happened inside my_span");


}
