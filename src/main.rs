use clap::Parser;
use std::convert::Infallible;
use hyper::{Body, Request, Response, Server};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};

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

impl Proxy {
    pub fn new(port: u16) -> Self {
        Proxy {
            port,
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr = ([0, 0, 0, 0], self.port).into();

        println!("Running proxy on: {}", &addr);

        let make_svc = make_service_fn(move |socket: &AddrStream| {
            let remote_addr = socket.remote_addr();
            println!("Handling connection for IP: {}", &remote_addr);

            async move {
                Ok::<_, Infallible>(service_fn(move |_: Request<Body>| async move {
                    Ok::<_, Infallible>(
                        Response::new(Body::from(format!("Hello, {}!", remote_addr)))
                    )
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
