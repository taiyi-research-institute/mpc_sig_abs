use std::collections::{BTreeMap, BTreeSet}; // To ensure that keys are in deterministic order.
use std::fmt::Display;

use super::MpcAddr;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait Messenger {
    type ErrorType: Display + Send + Sync + 'static;

    async fn send<T>(
        &self,
        topic: &str,
        src: MpcAddr,
        dst: MpcAddr,
        seq: usize,
        obj: &T,
    ) -> Result<(), Self::ErrorType>
    where
        T: Serialize + DeserializeOwned + Send + Sync;

    async fn receive<T>(
        &self,
        topic: &str,
        src: MpcAddr,
        dst: MpcAddr,
        seq: usize,
    ) -> Result<T, Self::ErrorType>
    where
        T: Serialize + DeserializeOwned + Send + Sync;
}

#[async_trait]
pub trait BatchMessenger: Messenger {
    /// Super::send() registers the messages to send, in some inner data structure, say, `logs`.
    /// Self::execute_send(&self) sends the messages.
    async fn execute_send(&self) -> Result<(), Self::ErrorType>;

    /// Self::execute_receive(&self) receive the messages with indices given by Super::send().
    /// The received messages are stored in `logs`.
    /// Then, call Super::receive() to get the messages.
    async fn execute_receive(&self) -> Result<(), Self::ErrorType>;

    /// Clear the `logs`.
    async fn clear(&self) -> Result<(), Self::ErrorType>;
}
