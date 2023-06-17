use crate::args::Args;
use crate::client_factory::ClientFactory;
use sdk::error::Error;
use sdk::streams::create_stream::CreateStream;
use sdk::streams::get_streams::GetStreams;
use sdk::topics::create_topic::CreateTopic;
use std::sync::Arc;
use tracing::info;

pub async fn init_streams(
    client_factory: Arc<dyn ClientFactory>,
    args: Arc<Args>,
) -> Result<(), Error> {
    let start_stream_id = args.get_start_stream_id();
    let topic_id: u32 = 1;
    let partitions_count: u32 = 1;
    let client = client_factory.create_client(args.clone()).await;
    let streams = client.get_streams(&GetStreams {}).await?;
    for i in 1..=args.streams {
        let stream_id = start_stream_id + i;
        if streams.iter().all(|s| s.id != stream_id) {
            info!("Creating the test stream {}", stream_id);
            let name = format!("stream {}", stream_id);
            client
                .create_stream(&CreateStream { stream_id, name })
                .await?;

            info!(
                "Creating the test topic {} for stream {}",
                topic_id, stream_id
            );
            let name = format!("topic {}", topic_id);
            client
                .create_topic(&CreateTopic {
                    stream_id,
                    topic_id,
                    partitions_count,
                    name,
                })
                .await?;
        }
    }
    Ok(())
}
