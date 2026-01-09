



use futures::StreamExt;
use libp2p::{
    gossipsub,
    identity,
    mdns,
    noise,
    swarm::{self, NetworkBehaviour, Swarm, SwarmEvent},
    tcp, yamux,
    Multiaddr, PeerId, Transport,
};
use tokio::io::{self, AsyncBufReadExt};

#[derive(NetworkBehaviour)]
struct GhostBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

let mut connected_peers: usize = 0;

    // ---- Identity ----
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Agent ID: {peer_id}");

    // ---- Transport ----
    let transport = tcp::tokio::Transport::new(tcp::Config::default())
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(noise::Config::new(&id_keys)?)
        .multiplex(yamux::Config::default())
        .boxed();

    // ---- Gossipsub ----
    let mut gossipsub = gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(id_keys),
        gossipsub::Config::default(),
    )?;

    let topic = gossipsub::IdentTopic::new("mission-channel");
    gossipsub.subscribe(&topic)?;

    // ---- mDNS ----
    let mdns = mdns::tokio::Behaviour::new(
        mdns::Config::default(),
        peer_id,
    )?;

    // ---- Behaviour ----
    let behaviour = GhostBehaviour { gossipsub, mdns };

    let mut swarm = Swarm::new(
        transport,
        behaviour,
        peer_id,
        swarm::Config::with_tokio_executor(),
    );

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse::<Multiaddr>()?)?;

    let mut stdin = io::BufReader::new(io::stdin()).lines();

    println!("Waiting for peers...");

    loop {
        tokio::select! {
            line = stdin.next_line() => {
                if let Ok(Some(msg)) = line {
               if connected_peers == 0 {
    println!("No peers connected yet. Message not sent.");
} else {
    if let Err(e) = swarm
        .behaviour_mut()
        .gossipsub
        .publish(topic.clone(), msg.into_bytes())
    {
        println!("Failed to send message: {:?}", e);
    }
}

                }
            }

            event = swarm.select_next_some() => match event {
               SwarmEvent::Behaviour(event) => match event {
    GhostBehaviourEvent::Gossipsub(
        gossipsub::Event::Message { message, .. }
    ) => {
        println!(
            "Received: {}",
            String::from_utf8_lossy(&message.data)
        );
    }

    GhostBehaviourEvent::Gossipsub(_) => {
        // Other gossipsub events like Subscribed/Unsubscribed
        // are intentionally ignored for now
    }

    GhostBehaviourEvent::Mdns(
    mdns::Event::Discovered(peers)
) => {
    for (peer, _) in peers {
        swarm
            .behaviour_mut()
            .gossipsub
            .add_explicit_peer(&peer);

        connected_peers += 1;
        println!("Peer connected: {peer}");
        println!("Connected peers: {connected_peers}");
    }
}


    GhostBehaviourEvent::Mdns(
    mdns::Event::Expired(peers)
) => {
    for (peer, _) in peers {
        swarm
            .behaviour_mut()
            .gossipsub
            .remove_explicit_peer(&peer);

        if connected_peers > 0 {
            connected_peers -= 1;
        }

        println!("Peer disconnected: {peer}");
        println!("Connected peers: {connected_peers}");
    }
}

} ,
                _ => {}
            }
        }
    }
}
