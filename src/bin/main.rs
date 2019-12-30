use cortex_gateway::{router::{EvmapRouter, Router}, Handler};
use pretty_env_logger;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Server, Method, Request, Body, Error};
use std::sync::Arc;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    let mut evmap_router: EvmapRouter = EvmapRouter::default();
    // setup routes? perhaps spawn process monitoring/setting up routes?
    evmap_router.add(Method::GET, "/", Handler { topic: String::from("/") });
    
    let router = Arc::new(evmap_router);

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
                async move {
                    router.handle(req).await
                }
            }))
        }
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}