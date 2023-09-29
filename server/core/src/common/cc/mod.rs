pub mod store;
pub mod user;
// redis = { version = "0.23.3", features = [ "cluster-async", "tokio-std-comp" ] }

use redis::cluster::ClusterClient;
use redis::Commands;

fn fetch_an_integer() -> String {
    let nodes = vec!["redis://127.0.0.1/","redis://127.0.0.1/", "redis://127.0.0.1/"];
    let client = ClusterClient::new(nodes).unwrap();
    let mut connection = client.get_connection().unwrap();
    let _: () = connection.set("test", "test_data").unwrap();
    let rv: String = connection.get("test").unwrap();
    return rv;
}
