use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};



pub async fn add_folder(client: &IpfsClient,path: &str) -> String{
    println!("\nadding folder {} to ipfs\n", path);
    let ipfs_hash = client.add_path(path).await.unwrap().last().unwrap().hash.clone();
    println!("folder hash : {}", ipfs_hash);
    ipfs_hash
}

pub async fn ipns_publish(client: &IpfsClient, ipfs_hash: &str) -> String {
    println!("\npublishing path {} to ipns\n", ipfs_hash);
    let folder_ipfs_path = format!("/ipfs/{}", ipfs_hash);
    let ipns_hash = client.name_publish(&folder_ipfs_path, false, None, None, None).await.unwrap().value.clone();
    println!("ipns hash : {}", ipns_hash);
    ipns_hash
}

pub async fn pin_list(client: &IpfsClient) {
    println!("listing pins");
    println!("{:#?}", client.pin_ls(None, None).await.unwrap());
}

pub async fn rm_all_pins(client: &IpfsClient) {
    println!("Deleting all pins");
    let pins = client.pin_ls(None, None).await.unwrap();
    for pin in pins.keys {
        client.pin_rm(&format!("/ipfs/{}", pin.0), true).await;
    }
}

pub async fn add_pin(client: &IpfsClient, ipfs_hash: &str) {
    println!("Pinning {}", ipfs_hash);
    client.pin_add(ipfs_hash, true).await;
}

pub async fn ls_remote_pins(client: &IpfsClient, service: &str) {
    println!("listing all remote pins");
    println!("{}", client.pin_remote_ls(service, None, None, None).await.unwrap());
}

pub async fn add_remote_pin(client: &IpfsClient, path: &str, service: &str, pin_name: Option<&str>) {
    println!("adding remote pin {}", path);
    println!("{:#?}", client.pin_remote_add(path, service, pin_name, None).await);
}

pub async fn ls_remote_pin_service(client: &IpfsClient, stat: Option<bool>) {
    println!("listing all remote pin services");
    println!("{:#?}", client.pin_remote_service_ls(stat).await);
}