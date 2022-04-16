use crate::matrix_capnp::message_event;
use capnp::serialize_packed;
use clap::Parser;
use color_eyre::eyre::{eyre, Result};
use std::{fs::File, io::BufReader};

pub mod matrix_capnp {
    include!(concat!(env!("OUT_DIR"), "/matrix_capnp.rs"));
}

pub mod matrix {
    use crate::matrix_capnp::message_event;
    use capnp::message::{Builder, HeapAllocator};

    pub fn build_event() -> Builder<HeapAllocator> {
        let mut message = Builder::new_default();
        {
            let mut event = message.init_root::<message_event::Builder>();

            {
                event.set_event_id("$ip5l5n9ckymoybrwh-jl8BDYl8HCv2nO1d4NCFp4ek0");
                event.set_origin_server_ts(1650133746117);
                event.set_sender("@mtrnord:nordgedanken.dev");
                event.set_room_id("!KwXDovBFhYakswlOwN:nordgedanken.dev");
                let mut unsigned = event.reborrow().init_unsigned();
                unsigned.set_age(405);
                unsigned.set_transaction_id("m1650132888916.13");
                let content = event.init_content();
                let mut text = content.init_text();
                text.set_body("This is a test");
            }
        }

        message
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    write: bool,
    #[clap(short, long)]
    read: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    if args.read && args.write || !args.read && !args.write {
        return Err(eyre!("Either read or write must be specified"));
    }

    if args.read {
        let mut data_file = File::open("./data")?;
        let mut buf_reader = BufReader::new(&mut data_file);
        let message_reader = serialize_packed::read_message(
            &mut buf_reader,
            ::capnp::message::ReaderOptions::new(),
        )?;

        let event = message_reader.get_root::<message_event::Reader>()?;

        println!("event_id: {}", event.get_event_id()?);
        println!("origin_server_ts: {}", event.get_origin_server_ts());
        match event.get_content().which() {
            Ok(message_event::content::Text(content)) => {
                println!("text body: {}", content.get_body()?);
                if content.has_formatted_body() {
                    println!("text formatted_body: {}", content.get_formatted_body()?)
                }
            }
            Err(err) => println!("content: {:?}", err),
            _ => println!("content type not yet supported"),
        }
    } else if args.write {
        let event = matrix::build_event();
        let mut data_file = File::create("./data")?;

        serialize_packed::write_message(&mut data_file, &event)?;

        println!("Wrote event to ./data");
    }

    Ok(())
}
