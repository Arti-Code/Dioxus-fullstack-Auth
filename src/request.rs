#![allow(unused)]
use dioxus::{logger::tracing, prelude::*};
//use std::io::Error;
use reqwest::{Error, *};
use ::reqwest::{
    *,
};
use serde::{Deserialize, Serialize};
use crate::user::*;
use anyhow::Result;



pub async fn get_user() -> anyhow::Result<UserResponse> {
    //let resp = match client.get("http://127.0.0.1:3000/profile").send().await {
    let resp = match reqwest::get("http://127.0.0.1:3000/profile").await {
        Ok(resp) => {
            //let msg = resp.text().await.unwrap().clone();
            //tracing::debug!(msg);
            return Ok(resp.json::<UserResponse>().await.unwrap());
            //return Ok(UserResponse::default());
        },
        Err(e) => {
            println!("error: {}", e);
            return Err(anyhow::anyhow!("Error: {}", e));
        },
    };
    return Ok(resp);
}

pub async fn register(name: String, password: String) -> anyhow::Result<()> {
    let post = UserEntry {
        username: name,
        password,
    };
    let url = format!("http://127.0.0.1:3000/register");
    println!("reqwest: {}", post.username.clone());
    match reqwest::Client::new().post("http://127.0.0.1:3000/register").json::<UserEntry>(&post).send().await {
        Err(e) => {
            println!("error: {}", e);
            return Err(anyhow::anyhow!("Error: {}", e));
        },
        Ok(req) => { 
            match req.status() {
                StatusCode::OK => {
                    println!("User created");
                },
                _ => {
                    println!("Error: {}", req.status());
                    return Err(anyhow::anyhow!("Error: {}", req.status()));
                },
            }
        }
    }
    Ok(())
}

pub async fn login(name: String, password: String) -> Result<()> {
    let post = UserEntry {
        username: name,
        password,
    };
    let client = reqwest::ClientBuilder::new().build()?;
    let response = client.post("http://127.0.0.1:3000/login").json(&post).send().await?;
    response.headers()
        .iter()
        .for_each(|(key, value)| {
            tracing::debug!("Header: {}: {:?}", key, value);
        }
    );
    response.cookies()
        .for_each(|cookie| {
            tracing::debug!("Cookie: {:?}", cookie);
        }
    );
    Ok(())
}

/* pub async fn login3(client: Client, name: String, password: String) -> Result<()> {
    let post = UserEntry {
        username: name,
        password,
    };
    //let client = reqwest::ClientBuilder::new().cookie_store(true).build()?;
    client.post("http://127.0.0.1:3000/login").json(&post).send().await?;
    Ok(())
} */

/* pub async fn login2(name: String, password: String) -> Result<()> {
    let post = UserEntry {
        username: name,
        password,
    };
    let client = reqwest::ClientBuilder::new().cookie_store(true).build()?;
    client.post("http://127.0.0.1:3000/login").json(&post).send().await?;
    Ok(())
} */
