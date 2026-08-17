fn main() -> Result<(),anyhow::Error> {
    env_logger::init();

    log::warn!("Hello world!");

    neomekanix_core::init()
}
