use tracing::Level;

//
pub(crate) fn setupLogging() {
  tracing_subscriber::fmt()
    .with_max_level(Level::DEBUG)
    .init();
}
