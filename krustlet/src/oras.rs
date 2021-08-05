#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // Initialize the logger
    env_logger::init();
}

// fn make_store(config: &Config) -> Arc<dyn kubelet::store::Store + Send + Sync> {
//     let client = oci_distribution::Client::from_source(config);
//     let mut store_path = config.data_dir.join(".oci");
//     store_path.push("modules");
//     let file_store = Arc::new(FileStore::new(client, &store_path));

//     if config.allow_local_modules {
//         file_store.with_override(Arc::new(kubelet::store::fs::FileSystemStore {}))
//     } else {
//         file_store
//     }
// }

// fn notify_bootstrap(message: String) {
//     println!("BOOTSTRAP: {}", message);
// }
