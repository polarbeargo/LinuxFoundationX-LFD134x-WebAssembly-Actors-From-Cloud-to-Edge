#[macro_use]
extern crate log;

use serde::{Deserialize, Serialize};
use serde_json;
use wapc_guest::prelude::*;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_http_server as http;
use wasmcloud_actor_keyvalue as kv;
use wasmcloud_actor_logging as logging;

#[actor::init]
fn init() {
    http::Handlers::register_handle_request(handle_http_request);

    logging::enable_macros();
}

fn handle_http_request(req: http::Request) -> HandlerResult<http::Response> {
    info!("Handling request");
    let tokens: Vec<&str> = req.path.split("/").collect();
    // tokens will be  ["", "books", "{isbn}"]
    if tokens.len() < 2 || tokens[1] != "books" {
        return Ok(http::Response::bad_request());
    }

    match req.method.as_ref() {
        "POST" => add_book(req.body),
        "GET" if tokens.len() == 3 => get_book(tokens[2]),
        "PUT" if tokens.len() == 3 => update_book(req.body, tokens[2]),
        "DELETE" if tokens.len() == 3 => delete_book(tokens[2]),
        _ => Ok(http::Response::bad_request()),
    }
}

fn add_book(raw: Vec<u8>) -> HandlerResult<http::Response> {
    info!("Adding book to library");
    let book: Book = serde_json::from_slice(&raw)?;
    kv::default().set(book.isbn.to_string(), serde_json::to_string(&book)?, 0)?;
    Ok(http::Response::ok())
}

fn get_book(isbn: &str) -> HandlerResult<http::Response> {
    info!("Querying book {}", isbn);
    let v = kv::default().get(isbn.to_string())?;
    if v.exists {
        info!("Found book!");
        let book: Book = serde_json::from_str(&v.value)?;
        Ok(http::Response::json(&book, 200, "OK"))
    } else {
        Ok(http::Response::not_found())
    }
}

fn update_book(raw: Vec<u8>, isbn: &str) -> HandlerResult<http::Response> {
    info!("Updating book {}", isbn);
    let book: Book = serde_json::from_slice(&raw)?;
    kv::default().set(isbn.to_string(), serde_json::to_string(&book)?, 0)?;
    Ok(http::Response::ok())
}

fn delete_book(isbn: &str) -> HandlerResult<http::Response> {
    info!("Deleting book {}", isbn);
    kv::default().del(isbn.to_string())?;
    Ok(http::Response::ok())
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Book {
    pub isbn: String,
    pub title: String,
    pub description: String,
    pub price: u32,
}
