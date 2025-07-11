use clap::{Parser, Subcommand, arg};

#[derive(Parser)]
#[command(version, about)]
struct Arguments {
    #[arg(short, long)]
    brokers: String,

    #[command(subcommand)]
    namespace: Namespace,
}

#[derive(Subcommand)]
enum Namespace {
    /// Operate on Kafka topics
    Topics {
        #[command(subcommand)]
        topics_subcommand: TopicsSubcommand,
    },
}

#[derive(Subcommand)]
enum TopicsSubcommand {
    /// Create new topics
    Create {
        #[arg(short, long)]
        name: String,

        #[arg(short, long, default_value_t = 1)]
        partitions: i32,

        #[arg(short, long, default_value_t = 1)]
        replication: i32,
    },

    /// Delete existing topics
    Delete {
        #[arg(short, long)]
        name: String,
    },

    /// List topics
    List {
        #[arg(short, long, value_parser = humantime::parse_duration, default_value = "500ms")]
        timeout: std::time::Duration,
    },
}

#[tokio::main]
async fn main() -> Result<(), rdkafka::error::KafkaError> {
    let args = Arguments::parse();
    let admin: rdkafka::admin::AdminClient<_> = rdkafka::config::ClientConfig::new().set("bootstrap.servers", &args.brokers).create()?;

    match &args.namespace {
        Namespace::Topics { topics_subcommand } => match &topics_subcommand {
            TopicsSubcommand::Create {
                name,
                partitions,
                replication,
            } => {
                let topic = rdkafka::admin::NewTopic::new(&name, *partitions, rdkafka::admin::TopicReplication::Fixed(*replication));
                admin.create_topics(&[topic], &rdkafka::admin::AdminOptions::new()).await.map(|_| ())
            }
            TopicsSubcommand::Delete { name } => admin.delete_topics(&[name], &rdkafka::admin::AdminOptions::new()).await.map(|_| ()),
            TopicsSubcommand::List { timeout } => {
                let metadata = admin.inner().fetch_metadata(None, rdkafka::util::Timeout::After(*timeout));
                Ok(metadata?.topics().iter().for_each(|topic| println!("{}", topic.name())))
            }
        },
    }
}
