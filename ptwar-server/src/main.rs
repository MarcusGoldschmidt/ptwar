use ptwar::PTWar;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let mut gloop = PTWar::new();

    gloop.start().await;
}
