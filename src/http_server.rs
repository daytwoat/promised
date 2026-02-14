use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;


/*will be used for more than showing an html page */

async fn serve_quote(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let html = r#"
        <html>
            <head><title>HEHE BOIAY</title></head>
            <body>
                <h1>Promise!</h1>
                <p>BACK TO WORK MOTHYERFUKCER</p>
            </body>
        </html>
    "#;

    Ok(Response::new(Body::from(html)))
}



pub async fn run_http_server() {
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(serve_quote)) });

    let addr = ([127,0,0,1], 80).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("HTTP SERVER RUNNING ON http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
