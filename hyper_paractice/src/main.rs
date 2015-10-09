extern crate hyper;

use std::io::Write;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;

fn hello(_: Request, res: Response<Fresh>) {
    res.send(b"Hello World!").unwrap();
}

fn main() {
    Server::http("127.0.0.1:3000").unwrap().handle(hello);
}