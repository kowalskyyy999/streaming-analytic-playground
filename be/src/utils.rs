use rand::{distributions::Alphanumeric, Rng};
use rdkafka::{config::ClientConfig, producer::FutureProducer};

pub fn hash_password(password: &String) -> core::result::Result<(String, String), Box<dyn std::error::Error>> {
    let salt = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let ps = format!("{password}{salt}");

    let digest = md5::compute(ps);
    let hash = format!("{:x}",digest);

    Ok((hash, salt))
}

pub fn kafka_producer_connection() -> Result<FutureProducer, Box<dyn std::error::Error>> {
    let brokers = "localhost:9092";
    
    let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .unwrap();

    Ok(producer)
}