use std::collections::HashMap;

use super::super::metadata::Proxy;
use super::super::util::gen_proxy_url;

pub fn gen_proxy_env_vars(proxy: Option<Proxy>) -> HashMap<String, String> {
    let mut env_vars: HashMap<String, String> = HashMap::new();
    match proxy {
        Some(_proxy) => {
            let http_proxy = gen_proxy_url(&_proxy.scheme, &_proxy.host, _proxy.port);
            env_vars.insert("http_proxy".to_string(), http_proxy.clone());
            env_vars.insert("https_proxy".to_string(), http_proxy.clone());
        }
        None => {}
    }
    env_vars
}
