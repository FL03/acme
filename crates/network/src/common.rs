pub use constants::*;
pub use types::*;

mod constants {}

mod types {
    use libp2p::{self, core::{muxing::StreamMuxerBox, transport::Boxed}};

    pub use libp2p::PeerId;

    // Authenticated DH Keys
    pub type AuthNoiseKey = libp2p::noise::AuthenticKeypair<CryptoSpec>;
    // Boxed Transport
    pub type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;
    // Standard Network Encryption Specification
    pub type CryptoSpec = libp2p::noise::X25519Spec;
    pub type Kad = libp2p::kad::Kademlia<libp2p::kad::store::MemoryStore>;
    // Wrapper for Multiaddr
    pub type NetworkAddress = libp2p::Multiaddr;
    // Wrapper for Noise Keypair
    pub type NoiseKey = libp2p::noise::Keypair<CryptoSpec>;
    pub type PeerKey = libp2p::identity::Keypair;
}

