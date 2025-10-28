# Rust Chat Server

Peer-to-peer chat playground built with [iroh](https://github.com/n0-computer/iroh).  
Each participant runs a local client that connects directly to peers in the same topic, shares a short "ticket" to invite others, and exchanges JSON-encoded chat messages via iroh's gossip protocol.

## What This App Does
- Spawns a local iroh endpoint and joins a gossip topic (either newly opened or provided through a ticket).
- Broadcasts chat lines you enter on stdin to all peers in the topic.
- Shares a Base32-encoded ticket containing the topic ID and your endpoint address so others can join you.
- Learns the display names that peers announce with an `AboutMe` message and uses them when printing chat output.

## Running It
1. Install Rust (stable toolchain) and fetch dependencies: `cargo fetch`.
2. Open a terminal for the first peer and run:
   ```bash
   cargo run -- --name Alice open
   ```
   This creates a new topic and prints a ticket similar to:
   ```
   > opening chat room for topic A4U...
   > our endpoint id: 7hR...
   > ticket to join us: m4k...
   ```
   Keep this process running.
3. Share the ticket string with a friend (or use another terminal on your machine) and join:
   ```bash
   cargo run -- --name Bob join --ticket m4k...
   ```
4. Type messages and press Enter. Every peer sees lines formatted as `<name>: <message>`.

Optional flags:
- `--name <string>` sets the display name advertised to other peers.
- `--bind-port <port>` is parsed today but not yet wired in; the endpoint currently binds to an OS-assigned port.

## P2P Messaging with iroh
`iroh` is a networking toolkit that combines content-addressed storage with transport multiplexing.  
This project uses:
- `iroh::Endpoint` to create an addressable node with a stable peer ID.
- `iroh-gossip` to publish/subscribe to topics over QUIC without relying on a centralized server.
- `Router::builder(...).accept(iroh_gossip::ALPN, gossip.clone())` to register the gossip protocol on the endpoint.

The workflow:
1. A peer opens a topic by generating a random `TopicId`.
2. Joining peers decode the shared ticket, learn the topic, and attempt to connect directly to the listed endpoints.
3. Once connected, each peer sends and receives events on the gossip stream. Messages are serialized as JSON, signed with a random nonce, and broadcast to everyone subscribed to the topic.
4. Because all communication flows over iroh, peers discover each other, negotiate connections, and relay chat payloads without a traditional server in the middle.

## Project Structure
- `src/main.rs` — CLI entrypoint: parses arguments, manages the gossip subscription, and runs I/O loops.
- `src/messages.rs` — Defines the JSON-serializable message types exchanged over gossip.
- `src/tickets.rs` — Serializes/deserializes invitation tickets with Base32 encoding for easy sharing.

## Next Steps
- Add persistence so late joiners can see earlier messages.
- Hook up `--bind-port` to support firewalled environments.
- Package binaries or Docker images for easier distribution.
