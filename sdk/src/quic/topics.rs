use crate::binary;
use crate::client::TopicClient;
use crate::error::Error;
use crate::quic::client::QuicClient;
use crate::topic::{Topic, TopicDetails};
use crate::topics::create_topic::CreateTopic;
use crate::topics::delete_topic::DeleteTopic;
use crate::topics::get_topic::GetTopic;
use crate::topics::get_topics::GetTopics;
use async_trait::async_trait;

#[async_trait]
impl TopicClient for QuicClient {
    async fn get_topic(&self, command: &GetTopic) -> Result<TopicDetails, Error> {
        binary::topics::get_topic(self, command).await
    }

    async fn get_topics(&self, command: &GetTopics) -> Result<Vec<Topic>, Error> {
        binary::topics::get_topics(self, command).await
    }

    async fn create_topic(&self, command: &CreateTopic) -> Result<(), Error> {
        binary::topics::create_topic(self, command).await
    }

    async fn delete_topic(&self, command: &DeleteTopic) -> Result<(), Error> {
        binary::topics::delete_topic(self, command).await
    }
}
