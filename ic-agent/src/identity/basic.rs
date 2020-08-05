use crate::{AgentError, Blob, Identity, Principal, RequestId, Signature};
use ring::signature::{Ed25519KeyPair, KeyPair};

pub struct BasicIdentity {
    key_pair: Ed25519KeyPair,
}

impl BasicIdentity {
    pub fn from_key_pair(key_pair: Ed25519KeyPair) -> Self {
        Self { key_pair }
    }
}

impl Identity for BasicIdentity {
    fn sender(&self) -> Result<Principal, AgentError> {
        Ok(Principal::self_authenticating(&self.key_pair.public_key()))
    }
    fn sign(&self, request: &RequestId, _principal: &Principal) -> Result<Signature, AgentError> {
        let signature = self.key_pair.sign(&request.to_vec());
        // At this point we shall validate the signature in this first
        // skeleton version.
        let public_key_bytes = self.key_pair.public_key().as_ref();

        Ok(Signature {
            signature: Blob::from(signature.as_ref()),
            public_key: Blob::from(public_key_bytes),
        })
    }
}