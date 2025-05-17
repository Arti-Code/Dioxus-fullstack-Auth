#![allow(unused)]
use std::collections::HashMap;

use dioxus::{logger::tracing, prelude::*};
//use std::io::Error;
use reqwest::{Error, *};
use ::reqwest::{
    *,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{user::*, MySession};
use anyhow::Result;



pub async fn get_user(session: MySession) -> anyhow::Result<UserResponse> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;
    let s = format!("session={}", session.0);
    tracing::debug!("{}", &s);
    let resp = match client.request(Method::GET, "http://127.0.0.1:3000/profile").header("Cookie", s).send().await {
        Ok(resp) => {
            return Ok(resp.json::<UserResponse>().await.unwrap());
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

pub async fn login(name: String, password: String) -> Result<String> {
    let post = UserEntry {
        username: name,
        password,
    };
    let mut session = String::new();
    let client = reqwest::ClientBuilder::new().build()?;
    let response = client.post("http://127.0.0.1:3000/login").json(&post).send().await?;
    for cookie in response.cookies().into_iter() {
        let k = cookie.name().to_string();
        let v = cookie.value().to_string();
        if k.contains("session") {
            session = v.clone();
        }
    }
    Ok(session.to_owned())
}

