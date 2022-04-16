use clap::Parser;
use color_eyre::eyre::{eyre, Result};
use minicbor::display;
use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::events::{
    common::Unsigned,
    message::{Event, EventType, MessageType, TextContent},
};

mod events;

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
        todo!();
    } else if args.write {
        let data: Event<TextContent> = Event {
            event_type: EventType::RoomMessage,
            content: TextContent {
                msg_type: MessageType::Text,
                body: "This is a test".into(),
                format: None,
                formatted_body: None,
            },
            room_id: "!KwXDovBFhYakswlOwN:nordgedanken.dev".into(),
            event_id: "$ip5l5n9ckymoybrwh-jl8BDYl8HCv2nO1d4NCFp4ek0".into(),
            sender: "@nordgedanken:nordgedanken.dev".into(),
            origin_server_ts: 1650133746117,
            unsigned: Unsigned {
                age: 405,
                transaction_id: "m1650132888916.13".into(),
                prev_content: None,
                redacted_because: None,
            },
            state_key: None,
        };
        {
            let mut data_file = File::create("data.cbor")?;
            minicbor::encode(&data, &mut data_file)?;
        }
        {
            let file = File::open("data.cbor")?;
            let mut reader = BufReader::new(file);
            let mut buffer = Vec::new();

            // Read file into vector.
            reader.read_to_end(&mut buffer)?;

            println!("{}", display(&buffer));
        }
    }

    Ok(())
}
