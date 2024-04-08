use std::fmt::Display;

use super::MpcAddr;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait Messenger {
    type ErrorType: Display + Send + Sync + 'static;

    async fn send<T>(
        &mut self,
        topic: &str,
        src: MpcAddr,
        dst: MpcAddr,
        seq: usize,
        obj: &T,
    ) -> Result<(), Self::ErrorType>
    where
        T: Serialize + DeserializeOwned + Send + Sync;

    async fn receive<T>(
        &mut self,
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
    async fn execute_send(&mut self) -> Result<(), Self::ErrorType>;
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
        &mut self,
        topic: &str,
        src: MpcAddr,
        dst: MpcAddr,
        seq: usize,
    ) -> Result<T, Self::ErrorType>
    where
        T: Serialize + DeserializeOwned + Send + Sync;
    async fn clear_receive(&mut self);

    async fn register<T>(
        &mut self,
        topic: &str,
        src: MpcAddr,
        dst: MpcAddr,
        seq: usize,
        obj: &T,
    ) -> Result<(), Self::ErrorType>
    where
        T: Serialize + DeserializeOwned + Send + Sync,
    {
        self.register_send(topic, src, dst, seq, obj).await?;
        self.register_receive(topic, dst, src, seq).await?;
        Ok(())
    }

    async fn execute(&mut self) -> Result<(), Self::ErrorType> {
        self.execute_send().await?;
        self.execute_receive().await?;
        Ok(())
    }

    async fn clear(&mut self) {
        self.clear_send().await;
        self.clear_receive().await;
    }
}
