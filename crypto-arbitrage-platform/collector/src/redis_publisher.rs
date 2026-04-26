use redis::{AsyncCommands, Client};

pub struct RedisPublisher {
    client: Client,
    streams_prefix: String,
}

impl RedisPublisher {
    pub async fn new(config: &crate::config::RedisConfig) -> anyhow::Result<Self> {
        let host = &config.host;
        let port = config.port;
        let addr = if let Some(ref password) = config.password {
            format!("redis://:{}@{}:{}", password, host, port)
        } else {
            format!("redis://{}:{}", host, port)
        };

        let client = Client::open(addr)?;
        Ok(Self {
            client,
            streams_prefix: config.streams_prefix.clone(),
        })
    }

    pub async fn publish(
        &self,
        topic: &str,
        payload: &[u8],
    ) -> anyhow::Result<()> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let stream_key = format!("{}:{}", self.streams_prefix, topic);
        let _: String = con.xadd(&stream_key, "*", &[("data", payload)]).await?;
        Ok(())
    }

    pub fn stream_key(&self, topic: &str) -> String {
        format!("{}:{}", self.streams_prefix, topic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_publisher_creation() {
        let config = crate::config::RedisConfig {
            host: "localhost".into(),
            port: 6379,
            password: None,
            streams_prefix: "collector".into(),
        };
        let publisher = RedisPublisher::new(&config).await;
        assert!(publisher.is_ok());
    }

    #[test]
    fn test_stream_key_format() {
        let config = crate::config::RedisConfig {
            host: "localhost".into(),
            port: 6379,
            password: None,
            streams_prefix: "collector".into(),
        };
        let publisher = RedisPublisher {
            client: Client::open("redis://localhost:6379").unwrap(),
            streams_prefix: config.streams_prefix.clone(),
        };
        assert_eq!(publisher.stream_key("binance:tier1"), "collector:binance:tier1");
    }
}