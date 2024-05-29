use ethers::{
    signers::Signer,
    types::{
        transaction::{eip2718::TypedTransaction, eip712::Eip712},
        Address, Signature,
    },
};
use ethers_gcp_kms_signer::GcpKmsSigner;

use crate::Error;

#[derive(Debug)]
pub struct KmsSigner {
    signer: GcpKmsSigner,
}

impl KmsSigner {
    pub fn new(signer: GcpKmsSigner) -> Self {
        Self { signer }
    }

    pub async fn sign_message<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<Signature, Error> {
        self.signer.sign_message(message).await.map_err(Error::Gcp)
    }

    pub async fn sign_transaction(&self, tx: &TypedTransaction) -> Result<Signature, Error> {
        self.signer.sign_transaction(tx).await.map_err(Error::Gcp)
    }

    pub async fn sign_typed_data<T: Eip712 + Send + Sync>(
        &self,
        payload: &T,
    ) -> Result<Signature, Error> {
        self.signer
            .sign_typed_data(payload)
            .await
            .map_err(Error::Gcp)
    }

    pub fn address(&self) -> Address {
        self.signer.address()
    }

    pub fn chain_id(&self) -> u64 {
        self.signer.chain_id()
    }

    pub fn with_chain_id<T: Into<u64>>(mut self, chain_id: T) -> Self {
        self.signer = self.signer.with_chain_id(chain_id);
        self
    }
}