use crate::primitives::{AuthenticatedStaticKeys, StaticKeys};
use libp2p::{
    core::{
        muxing::StreamMuxerBox,
        transport::Boxed,
        upgrade,
    },
    identity,
    mplex,
    noise,
    PeerId,
    tcp,
    Transport,
};

#[derive(Clone, Debug)]
pub struct Peer {
    pub id: PeerId,
    pub key: identity::Keypair
}

impl Peer {
    pub fn new() -> Self {
        let key = identity::Keypair::generate_ed25519();
        let id = PeerId::from(key.public().clone());

        Self {
            id: id.clone(),
            key: key.clone()
        }
    }

    pub fn from(key: identity::Keypair) -> Self {
        Self {
            id: PeerId::from(key.public().clone()),
            key: key.clone()
        }
    }

    pub fn authorize(&self) -> AuthenticatedStaticKeys {
        let dh_keys = StaticKeys::new()
            .into_authentic(&self.key.clone())
            .expect("Signing Error: Failed to sign the static DH KeyPair");
        return dh_keys.clone()
    }

    pub fn transport(&self) -> Boxed<(PeerId, StreamMuxerBox)> {
        let transport = tcp::TokioTcpConfig::new()
            .nodelay(true)
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(self.authorize()).into_authenticated())
            .multiplex(mplex::MplexConfig::new())
            .boxed();
        return transport
    }
}

impl std::fmt::Display for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Peer(id={})", self.id)
    }
}