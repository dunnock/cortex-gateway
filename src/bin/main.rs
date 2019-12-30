use cortex_gateway::{
    router::{EvmapRouter, EvmapRouterWriter, Router},
    Handler,
};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Request, Server};
use pretty_env_logger;
use std::sync::Arc;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "cortex", about = "HTTP gateway for grayarea functions")]
struct Cli {
    /// port to start server
    #[structopt(short = "p", long = "port", default_value = "3000")]
    port: u16,
    /// server configuration (yaml)
    #[structopt(short = "c", long = "config", default_value = "config.yaml")]
    config: std::path::PathBuf,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Cli::from_args();
    pretty_env_logger::init();

    let (map_r, map_w) = evmap::new();
    let mut routes_writer = EvmapRouterWriter::new(map_w);
    // setup routes? perhaps spawn process monitoring/setting up routes?
    routes_writer.add(
        Method::GET,
        "/",
        Handler {
            id: 0,
            path: String::from("/"),
            topic: String::from("root"),
        },
    );

    let router = Arc::new(EvmapRouter::new(map_r));

    // TODO: to change routes in parallel thread perhaps we will need to spawn a new thread
    // which will own the write handle to evmap

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.

        // create router reference per connection
        let router = router.clone();
        async {
            Ok::<_, Error>(service_fn(move |req: Request<Body>| {
                // for every request we have to copy a handle to list of routes
                // not the most efficient way :-)
                let router = router.clone();
                async move { router.handle(req).await }
            }))
        }
    });

    let addr = ([127, 0, 0, 1], args.port).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
