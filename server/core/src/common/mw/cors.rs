// use axum::{
//     http::{header, HeaderMap, Method, Request, StatusCode},
//     middleware::{self, Next},
//     response::{IntoResponse, Response},
//     routing::{get, post},
//     Json, Router,
// };

// pub async fn cors<B>(req: Request<B>, resp: Response, nex: Next<B>) -> Result<Response, StatusCode> {
//     let origin = req.headers().get("origin");
//     match origin {
//         Some(val) => {
//             let header_mut = req.headers_mut();
//             header_mut.insert("Access-Control-Allow-Origin", val.parse().unwrap());
//             header_mut.insert("Access-Control-Allow-Methods", "POST, GET, OPTIONS, PUT, DELETE, UPDATE".parse().unwrap());
//             header_mut.insert(
//                 "Access-Control-Allow-Headers",
//                 "Origin, X-Requested-With, Content-Type, Accept, Authorization".parse().unwrap(),
//             );
//             header_mut.insert(
//                 "Access-Control-Expose-Headers",
//                 "Content-Length, Access-Control-Allow-Origin, Access-Control-Allow-Headers, Cache-Control, Content-Language, Content-Type"
//                     .parse()
//                     .unwrap(),
//             );
//             header_mut.insert("Access-Control-Allow-Credentials", "true".parse().unwrap());
//             header_mut.insert("Access-Control-Max-Age", "86400".parse().unwrap());
//             if req.method().as_str() == "OPTIONS" {
//                 return Ok(StatusCode::NO_CONTENT);
//             }

//             next.run(req).await
//         }
//         None => Ok(next.run(req).await),
//     }
// }
