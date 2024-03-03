use std::io;

use reqwest::Client;
use serde::de::DeserializeOwned;
use tauri::AppHandle;

use super::global_config::get_global_config;

pub fn get_reqwest_client(app_handle: &AppHandle) -> Result<Client, io::Error> {
    let global_config = get_global_config(app_handle)?;

    let proxy = if let Some(proxy) = global_config.proxy {
        if !proxy.is_empty() {
            Some(reqwest::Proxy::all(proxy).unwrap())
        } else {
            None
        }
    } else {
        None
    };

    if let Some(proxy) = proxy.clone() {
        Ok(reqwest::Client::builder().proxy(proxy).build().unwrap())
    } else {
        Ok(reqwest::Client::new())
    }
}

pub async fn get_data<T>(client: Client, url: String) -> Result<T, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    Ok(client.get(url).send().await?.json::<T>().await?)
}
