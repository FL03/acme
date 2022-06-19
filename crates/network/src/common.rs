pub use constants::*;
pub use types::*;

mod constants {
    pub use crate::actors::constants::*;
    pub use crate::behaviours::constants::*;
    pub use crate::crypto::constants::*;
}

mod types {
    use libp2p::{self, core::{muxing::StreamMuxerBox, transport::Boxed}};

    pub use libp2p::PeerId;

    pub use crate::actors::constants::*;
    pub use crate::behaviours::types::*;
    pub use crate::crypto::types::*;

    // Authenticated DH Keys
    pub type AuthNoiseKey = libp2p::noise::AuthenticKeypair<CryptoSpec>;
    // Boxed Transport
    pub type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;
    // Standard Network Encryption Specification
    pub type CryptoSpec = libp2p::noise::X25519Spec;
    // Wrapper for Multiaddr
    pub type NetworkAddress = libp2p::Multiaddr;
    // Wrapper for Noise Keypair
    pub type NoiseKey = libp2p::noise::Keypair<CryptoSpec>;
    pub type PeerKey = libp2p::identity::Keypair;
}

