use std::fs;

pub(crate) fn get_server_address() -> String {
    let config = read_config();

    let host = read_config_property(config.clone(), "server.host");
    let port = read_config_property(config.clone(), "server.port");
    return format!("{host}:{port}");
}

pub(crate) fn is_masterhost() -> bool {
    let config = read_config();
    let is_masterhost = read_config_property(config.clone(), "server.ismasterhost");
    return is_masterhost.to_string() == "true"
}

pub(crate) fn get_sync_interval_ms() -> u64 {
    let config = read_config();
    let sync_interval_ms = read_config_property(config.clone(), "sync.interval.ms");
    return sync_interval_ms.parse().unwrap()
}

pub(crate) fn get_replicas_addr() -> Vec<String> {
    let config = read_config();
    let sync_interval_ms = read_config_property(config.clone(), "replicas.addr");
    return sync_interval_ms.split(",").map(|s| s.to_string()).filter(|s| !s.is_empty()).collect()
}

fn read_config() -> Vec<String> {
    fs::read_to_string("server.conf")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn read_config_property(config: Vec<String>, property_name: &str) -> String {
    let config_property_value = config
        .iter()
        .find(|&x| x.contains(property_name))
        .unwrap()
        .split("=")
        .collect::<Vec<_>>()[1];
    String::from(config_property_value)
}