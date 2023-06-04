



use std::sync::Arc;
use once_cell::sync::Lazy;
use rand::{Rng, SeedableRng, RngCore};
use rand_chacha::{rand_core, ChaCha12Rng};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use futures::StreamExt;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use log::{error, info};

mod s4;
use crate::s4::*;


/* 
    if we want to use Result<(), impl std::error::Error + Send + Sync + 'static>
    as the return type thus the error variable must be sepecified also the Error trait
    must be implemented for the error type (impl Error for ErrorType{}) since 
    we're implementing the Error trait for the error type in return type   
*/
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{


    /* start an async and concurrent server to handle socket packets from clients concurrently */ 
    start_server(|req, res| async move{
        Ok(
            Response{}
        )
    }).await



}
