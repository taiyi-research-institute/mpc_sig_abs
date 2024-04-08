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
        obj: &T,
    ) -> Result<(), Self::ErrorType>
    where
        T: Serialize + DeserializeOwned + Send + Sync;

    async fn batch_send<T>(
        &self,
        batch: &Vec<(String, MpcAddr, MpcAddr, T)>,
    ) -> Result<(), Self::ErrorType>
    where
        T: Serialize + DeserializeOwned + Send + Sync;

    async fn receive<T>(
        &self,
        topic: &str,
        src: MpcAddr,
        dst: MpcAddr,
    ) -> Result<T, Self::ErrorType>
    where
        T: Serialize + DeserializeOwned + Send + Sync;

    async fn batch_receive<T>(
        &self,
        batch: &Vec<(String, MpcAddr, MpcAddr)>,
    ) -> Result<Vec<(String, MpcAddr, MpcAddr, T)>, Self::ErrorType>
    where
        T: Serialize + DeserializeOwned + Send + Sync;

    async fn scatter<T>(
        &self,
        topic: &str,
        src: MpcAddr,
        dsts: &BTreeSet<MpcAddr>,
        obj: &T,
    ) -> Result<(), Self::ErrorType>
    where
        T: Serialize + DeserializeOwned + Send + Sync;

    async fn gather<T>(
        &self,
        topic: &str,
        srcs: &BTreeSet<MpcAddr>,
        dst: MpcAddr,
    ) -> Result<BTreeMap<MpcAddr, T>, Self::ErrorType>
    where
        T: Serialize + DeserializeOwned + Send + Sync;
}
