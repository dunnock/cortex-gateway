use crate::Handler;
use async_trait::async_trait;
use hyper::{service, Body, Error, Method, Request, Response};
use std::future::Future;
use std::pin::Pin;

#[derive(Copy)]
pub struct EvmapRouter {
    // evmap read and write handles
}

/// Thread safe router
/// - add/remove routes
/// - handle routes
#[async_trait]
pub trait Router: Send + Sync + Copy {
    fn add(&mut self, method: Method, route_pattern: &str, handler: Handler);
    fn remove(&mut self, route_pattern: &str);
    async fn handle(&self, req: Request<Body>) -> Result<Response<Body>, Error>;
}

impl Default for EvmapRouter {
    fn default() -> Self {
        Self {
	// init evmap read and write handles

		}
    }
}

impl Clone for EvmapRouter {
    fn clone(&self) -> Self {
        Self {
			// clone only read handle of evmap here, leaving write handle untouched
		}
    }
}

#[async_trait]
impl Router for EvmapRouter {
    fn add(&mut self, method: Method, route_pattern: &str, handler: Handler) {}
    fn remove(&mut self, route_pattern: &str) {}
    async fn handle(&self, req: Request<Body>) -> Result<Response<Body>, Error> {
        let response = async {
            //nats post/get
            "Hello, World".into()
        }
        .await;
        Ok(Response::new(response))
    }
}

/*
impl service::Service<Request<Body>> for EvmapRouter {
    type Response = Response<Body>;
    type Error = error::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        // create the body
        let resp = self.handle(req);

        // create a response in a future.
        let fut = async {
            Ok(resp)
        };

        // Return the response as an immediate future
        Box::pin(fut)
    }
}*/
