use{
    hyper::{
        Body, Client, Request, Response, Server, Uri,
        service::service_fn,
        rt::run,
    },
    futures::{
        compat::Future01CompatExt,
        future::{ FutureExt, TryFutureExt }
    },
    std::net::SocketAddr,
};

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
//    a rather bland response
    Ok(Response::new(Body::from("Hello world!")))
//    let's try a proxy...
//    how to get chrome to stop caching this page?
//    let url_str = "http://www.rust-lang.org/en-US/";
//    let url = url_str.parse::<Uri>().expect("Failed to parse.");
//    let res = Client::new().get(url).compat().await;
//    println!("Request finished! Printing:");
//    res
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    let serve_future = Server::bind(&addr)
//        .serve(|| service_fn(|req| serve_req(req).boxed().compat()));
        .serve(|| service_fn(|req| {
            dbg!(&req);
            serve_req(req).boxed().compat()
        }));


    if let Err(e) = serve_future.compat().await {
        eprintln!("Server error: {}", e)
    }
}

fn main() {
    let addr = SocketAddr::from(([127,0,0,1], 3000));

    let futures_03_future = run_server(addr);
    let futures_01_future = futures_03_future.unit_error().boxed().compat();

    run(futures_01_future);

}
