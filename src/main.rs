use clap::Parser;
use colored::*;
use futures::future::TryFutureExt;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::Client;
use hyper::{Body, Request, Response, Server};
use regex::Regex;
use std::convert::Infallible;
use std::result::Result;

mod config;

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

fn format_status(status: hyper::StatusCode) -> ColoredString {
    if status.is_success() {
        return status.to_string().green().bold();
    } else if (status.is_client_error() || status.is_server_error()) {
        return status.to_string().red().bold();
    } else if status.is_redirection() {
        return status.to_string().blue().bold();
    }
    return status.to_string().yellow().bold();
}

async fn proxy_inner(
    req: Request<Body>,
    config: config::Config,
    client: Client<hyper::client::HttpConnector>,
) -> Result<Response<Body>, hyper::Error> {
    // Await the response...

    let (parts, body) = req.into_parts();
    let path: String = parts
        .uri
        .path()
        .parse::<String>()
        .unwrap_or(String::from("/"));
    let method = parts.method;
    // let authority: &str = parts.uri.authority().unwrap().as_str();
    let path_and_query = parts.uri.path_and_query().unwrap().as_str();
    let headers = parts.headers;
    let request_host: &str = headers.get("host").map_or("none", |h| h.to_str().unwrap());
    // let host: &str = parts.uri.host().unwrap_or("defaulthost");
    let scheme: &str = parts.uri.scheme_str().unwrap_or("http");

    for (uri_regex, host_name) in config.rewrites.into_iter() {
        // println!("{} / {}", uri_regex, host_name);
        let r = Regex::new(&uri_regex).unwrap();

        let uri_str = [scheme, "://", request_host, path_and_query].join("");

        let is_match = r.is_match(&uri_str.as_str());

        let destination_host = config
            .hosts
            .get(&host_name)
            .unwrap()
            .parse::<hyper::Uri>()
            .unwrap();

        // println!("destination_host={:?}", destination_host);
        // println!(
        //     "regex={:?} host={} path={} scheme={} uri={:?}",
        //     r, request_host, path, scheme, parts.uri
        // );
        // println!("headers={:?}", headers);
        // println!("Testing regex against {}: is_match={}", uri_str, is_match);
        if (is_match) {
            let uri = hyper::Uri::builder()
                .scheme(destination_host.scheme().unwrap().as_str())
                .authority(destination_host.authority().unwrap().as_str())
                .path_and_query(path_and_query)
                .build()
                .unwrap();

            // println!("Requesting {:?}", uri);
            // let uri = Uri::new();

            let outgoing_request = Request::builder()
                .method(method)
                .uri(uri.clone())
                .body(body)
                .unwrap();

            return client
                .request(outgoing_request)
                .map_err(|e| {
                    eprintln!("{:?}", e);
                    e
                })
                .map_ok(|v| {
                    println!(
                        "| {} | {} => {} |",
                        format_status(v.status()),
                        uri_str.clone().to_string().yellow(),
                        uri.clone().to_string().green()
                    );
                    v
                })
                .await;
        }
    }
    Ok(hyper::Response::new(Body::from(
        "proxee error: No matching rule found.",
    )))
}

impl Proxy {
    pub fn new(port: u16) -> Self {
        Proxy { port }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr = ([0, 0, 0, 0], self.port).into();

        println!("Running proxy on: {}", &addr);
        let client = Client::new();

        let parsed_config = config::parse().unwrap_or_else({
            |e| {
                panic!("Error parsing configuration: {}", e);
            }
        });

        let make_svc = make_service_fn(move |socket: &AddrStream| {
            let remote_addr = socket.remote_addr();
            let client = client.clone();
            let config = parsed_config.clone();
            // println!("Handling connection for IP: {}", &remote_addr);

            async move {
                Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                    proxy_inner(req, config.clone(), client.clone())
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
