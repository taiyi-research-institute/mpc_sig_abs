use std::fmt::Display;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait BatchMessenger {
    type ErrorType: Display + Send + Sync + 'static;

    fn register_send<T>(
        &mut self,
        topic: &str,
        src: usize,
        dst: usize,
        seq: usize,
        obj: &T,
    ) -> Result<(), Self::ErrorType>
    where
        T: Serialize + DeserializeOwned + Send + Sync;
    async fn execute_send(&mut self) -> Result<(), Self::ErrorType>;
    fn clear_send(&mut self);

    fn register_receive(
        &mut self,
        topic: &str,
        src: usize,
        dst: usize,
        seq: usize,
    ) -> Result<(), Self::ErrorType>;
    async fn execute_receive(&mut self) -> Result<(), Self::ErrorType>;
    fn unpack_receive<T>(
        &mut self,
        topic: &str,
        src: usize,
        dst: usize,
        seq: usize,
    ) -> Result<T, Self::ErrorType>
    where
        T: Serialize + DeserializeOwned + Send + Sync;
    fn clear_receive(&mut self);

    fn register<T>(
        &mut self,
        topic: &str,
        src: usize,
        dst: usize,
        seq: usize,
        obj: &T,
    ) -> Result<(), Self::ErrorType>
    where
        T: Serialize + DeserializeOwned + Send + Sync,
    {
        self.register_send(topic, src, dst, seq, obj)?;
        self.register_receive(topic, dst, src, seq)?;
        Ok(())
    }

    async fn execute(&mut self) -> Result<(), Self::ErrorType> {
        self.execute_send().await?;
        self.execute_receive().await?;
        Ok(())
    }

    fn clear(&mut self) {
        self.clear_send();
        self.clear_receive();
    }
}
