use http::{header::HeaderMap, Request};
use std::task::{Context, Poll};
use tower::retry::Policy;
use tower::{Layer, Service};

// pub struct RetryPolicy<T>(u8);

// impl<T, E> Policy<Request<T>, Response<T>, E> for RetryPolicy<T> {
//     type Future = future::Ready<()>;

//     fn retry(&mut self, req: &mut Req, result: &mut Result<Res, E>) -> Option<Self::Future> {
//         match result {
//             Ok(_) => {
//                 // Treat all `Response`s as success,
//                 // so don't retry...
//                 None
//             }
//             Err(_) => {
//                 // Treat all errors as failures...
//                 // But we limit the number of attempts...
//                 if self.0 > 0 {
//                     // Try again!
//                     self.0 -= 1;
//                     Some(future::ready(()))
//                 } else {
//                     // Used all our attempts, no retry...
//                     None
//                 }
//             }
//         }
//     }

//     fn clone_request(&mut self, req: &Req) -> Option<Req> {
//         Some(req.clone())
//     }
// }

#[derive(Debug)]
pub struct RequestHeadersLayer {
    headers: HeaderMap,
}

impl RequestHeadersLayer {
    pub fn new(headers: HeaderMap) -> Self {
        RequestHeadersLayer { headers }
    }
}

impl<S> Layer<S> for RequestHeadersLayer {
    type Service = RequestHeaders<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestHeaders {
            inner,
            headers: self.headers.clone(),
        }
    }
}

#[derive(Clone)]
pub struct RequestHeaders<S> {
    inner: S,
    headers: HeaderMap,
}

impl<Body, S> Service<Request<Body>> for RequestHeaders<S>
where
    S: Service<Request<Body>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        if !self.headers.is_empty() {
            req.headers_mut().extend(self.headers.clone());
        }

        self.inner.call(req)
    }
}
