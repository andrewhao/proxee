use clap::Parser;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::Client;
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::result::Result;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Port number to start proxy on
    #[clap(short, long, default_value_t = 8080)]
    port: u16,
}

struct Proxy {
    port: u16,
}

async fn proxy_inner(
    req: Request<Body>,
    client: Client<hyper::client::HttpConnector>,
) -> Result<Response<Body>, hyper::Error> {
    // Await the response...

    let (parts, body) = req.into_parts();
    let path: String = parts.uri.path().parse::<String>().unwrap_or_default();
    let host: &str = parts.uri.host().unwrap();
    let scheme: &str = parts.uri.scheme_str().unwrap();
    let headers = parts.headers;

    // let uri = ("https://www.wejoinin.com/" + req.uri().path()).parse?;
    let uri = "http://localhost:3000".parse().unwrap();
    client.get(uri).await
    // Ok(Response::builder().status(200).body(Body::empty()).unwrap())
}

impl Proxy {
    pub fn new(port: u16) -> Self {
        Proxy { port }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr = ([0, 0, 0, 0], self.port).into();

        println!("Running proxy on: {}", &addr);
        let client = Client::new();

        let make_svc = make_service_fn(move |socket: &AddrStream| {
            let remote_addr = socket.remote_addr();
            let client = client.clone();
            println!("Handling connection for IP: {}", &remote_addr);

            async move {
                Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                    proxy_inner(req, client.clone())
                }))
            }
        });

        let server = Server::bind(&addr).serve(make_svc);

        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let s = Proxy::new(args.port).run().await;
    s.unwrap()
}
