use iron::*;

// fn start_web() {
//     println!("Starting on http://localhost:6666");
//     Iron::new(get_form).http("http://localhost:6666").unwrap();
// }
//
// fn get_form(_req: &mut Request) -> IronResult<Response> {
//     let mut response = Response::new();
//
//     response.set_mut(status::Ok);
//     // response.set_mut(mime_type::utf8);
//     response.set_mut(r#"
//         <title>GCD calculation</title>
//         <form action="/gcd" method="post">
//         <input type="text" name="n" />
//         <input type="text" name="n" />
//         <button type="submit"> Compute GCD</button>
//         </form>
//     "#);
//     Ok(response);
// }