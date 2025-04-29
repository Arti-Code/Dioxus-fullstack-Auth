#![allow(unused)]
use dioxus::{prelude::*};
//use std::io::Error;
use reqwest::{Error, *};
use serde::{Deserialize, Serialize};
use crate::user::*;
use anyhow::Result;



pub async fn get_user() -> anyhow::Result<User> {
    let resp = reqwest::get("http://192.168.0.96:3000/protected")
        .await.unwrap().json::<User>().await.unwrap();
    return Ok(resp);
}

pub async fn register(name: String, password: String) -> anyhow::Result<()> {
    let post = UserEntry {
        username: name,
        password,
    };
    let url = format!("http://192.168.0.96:3000/register");
    println!("reqwest: {}", post.username.clone());
    match reqwest::Client::new().post("http://192.168.0.96:3000/register").json::<UserEntry>(&post).send().await {
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

/* pub async fn register2(name: String, password: String) -> Result<()> {
    let post = UserEntry {
        name,
        password,
    };
    let url = format!("http://192.168.0.96:3000/register");
    match reqwest::Client::new().post(url).json(&post).send().await? {
        Err(e) => {
            eprintln!("{}", e);
        },
        Ok(req) => { 
            match req.status() {
                StatusCode::OK => {
                    println!("User created");
                },
                _ => {
                    println!("Error: {}", req.status());
                },
            }
        }
    }
} */

pub async fn login(name: String, password: String) -> Result<()> {
    let post = UserEntry {
        username: name,
        password,
    };
    reqwest::Client::new().post("http://192.168.0.96:3000/login").json(&post).send().await?;
    Ok(())
}
