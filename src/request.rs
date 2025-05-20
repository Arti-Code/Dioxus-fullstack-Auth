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
    let client = reqwest::Client::builder().build()?;
    //let client = reqwest::Client::builder().cookie_store(true).build()?;
    let s = format!("[SESSION (get)]: {}", session.0);
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
    session = extract_session_from_response(&response);
    Ok(session.to_owned())
}

pub async fn logout(session: MySession) -> anyhow::Result<()> {
    let client = reqwest::Client::builder().build()?;
    //let client = reqwest::Client::builder().cookie_store(true).build()?;
    let s = format!("session={}", session.0);
    match client.request(Method::GET, "http://127.0.0.1:3000/logout").header("Cookie", s).send().await {
        Ok(resp) => {
            return Ok(());
        },
        Err(e) => {
            println!("error: {}", e);
            return Err(anyhow::anyhow!("Error: {}", e));
        },
    }
}


pub fn extract_session_from_response(resp: &Response) -> String {
    let mut session = String::new();
    resp.headers().get_all("set-cookie").iter().for_each(|header| {
        tracing::debug!("headers: {}", header.to_str().unwrap());
        if header.to_str().unwrap().contains("session") {
            let v = header.to_str().unwrap();
            tracing::debug!("header: {}", v);
            let s: Vec<&str> = v.split("; ").collect();
            s.iter().for_each(|h| {
                tracing::debug!("split1: {}", h);
            });
            let s1: Vec<&str> = s[0].split("=").collect();
            s1.iter().for_each(|h| {
                tracing::debug!("split2: {}", h);
            });
            session = s1[1].to_string();
        }
    });
    tracing::debug!("[SESSION (extract)]: {}", session);
    session
}