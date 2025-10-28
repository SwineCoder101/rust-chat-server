use std::collections::HashMap;

use anyhow::Result;
use clap::Parser;
use futures_lite::StreamExt;
use iroh::protocol::Router;
use iroh::Endpoint;
use iroh_gossip::{api::{Event, GossipReceiver}, net::Gossip, proto::TopicId};
mod messages;
mod tickets;
use crate::messages::{Message, MessageBody};
use crate::tickets::{Ticket};


#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    name: Option<String>,
    #[clap(short, long, default_value = "0")]
    bind_port: u16,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    Open,
    Join {
        ticket: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint = Endpoint::bind().await?;

    println!("> our endpoint id: {}", endpoint.id());
    let gossip = Gossip::builder().spawn(endpoint.clone());

    let router = Router::builder(endpoint.clone())
        .accept(iroh_gossip::ALPN, gossip.clone())
        .spawn();

    let id = TopicId::from_bytes(rand::random());
    let endpoint_ids = vec![];

    let topic = gossip.subscribe_and_join(id, endpoint_ids).await?;

    let ticket = {

    let me = endpoint.addr();
    let endpoints = vec![me];
    Ticket { topic: id, endpoints }
    };
    println!("> ticket to join us: {ticket}");


    let (sender, receiver) = topic.split();


    let message = Message::new(MessageBody::AboutMe {
        from: endpoint.id(),
        name: String::from("alice"),
    });

    sender.broadcast(message.to_vec().into()).await?;

    tokio::spawn(subscribe_loop(receiver));

    let (line_tx, mut line_rx) = tokio::sync::mpsc::channel(1);
    std::thread::spawn(move || input_loop(line_tx));


    println!("> type a message and hit enter to broadcast...");
    // listen for lines that we have typed to be sent from `stdin`
    while let Some(text) = line_rx.recv().await {
        // create a message from the text
        let message = Message::new(MessageBody::Message {
            from: endpoint.id(),
            text: text.clone(),
        });
        // broadcast the encoded message
        sender.broadcast(message.to_vec().into()).await?;
        // print to ourselves the text that we sent
        println!("> sent: {text}");
    }

    router.shutdown().await?;

    Ok(())
}

async fn subscribe_loop(mut receiver: GossipReceiver) -> Result<()> {
    let mut names = HashMap::new();
    while let Some(event) = receiver.try_next().await? {
        if let Event::Received(msg) = event {
            match Message::from_bytes(&msg.content)?.body {
                MessageBody::AboutMe { from, name } => {
                    names.insert(from, name.clone());
                    println!("> {} is now known as {}", from.fmt_short(), name);
                }
                MessageBody::Message { from, text } => {
                    let name = names
                        .get(&from)
                        .map_or_else(|| from.fmt_short().to_string(), String::to_string);
                    println!("{}: {}", name, text);
                }
            }
        }
    }
    Ok(())
}


fn input_loop(line_tx: tokio::sync::mpsc::Sender<String>) -> Result<()> {
    // create a new string buffer
    let mut buffer = String::new();
    // get a handle on `Stdin`
    let stdin = std::io::stdin(); // We get `Stdin` here.
    loop {
        // loop through reading from the buffer...
        stdin.read_line(&mut buffer)?;
        // and then sending over the channel
        line_tx.blocking_send(buffer.clone())?;
        // clear the buffer after we've sent the content
        buffer.clear();
    }
}
