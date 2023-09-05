use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>,
hyper::Error> {
    // This fn is called for each HTTP request
    let response_body = "Hello world!";
    let response = Response::new(Body::from(response_body));
    Ok(response)
}

#[tokio::main]
async fn main() {
    // Create a new HTTP server that listens on port 8080
    let addr = ([0, 0, 0, 0], 8080).into();
    let make_service = make_service_fn(|_conn| async {
        Ok::<_, hyper::Error>(service_fn(handle_request))
    });
    let server = Server::bind(&addr).serve(make_service);

    // Start the server and print a message to the console
    println!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
