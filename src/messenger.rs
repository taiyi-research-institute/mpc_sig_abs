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
pub trait BatchMessenger {
    type ErrorType: Display + Send + Sync + 'static;

    async fn register_send<T>(
        &mut self,
        topic: &str,
        src: MpcAddr,
        dst: MpcAddr,
        seq: usize,
        obj: &T,
    ) -> Result<(), Self::ErrorType>
    where
        T: Serialize + DeserializeOwned + Send + Sync;
    async fn execute_send(&self) -> Result<(), Self::ErrorType>;
    async fn clear_send(&mut self);

    async fn register_receive(
        &mut self,
        topic: &str,
        src: MpcAddr,
        dst: MpcAddr,
        seq: usize,
    ) -> Result<(), Self::ErrorType>;
    async fn execute_receive(&mut self) -> Result<(), Self::ErrorType>;
    async fn unpack_receive<T>(
        &self,
        topic: &str,
        src: MpcAddr,
        dst: MpcAddr,
        seq: usize,
    ) -> Result<T, Self::ErrorType>
    where
        T: Serialize + DeserializeOwned + Send + Sync;
    async fn clear_receive(&mut self);
}
