extern crate model;
use String;
use std::collections::HashMap;
use model::HttpRequest;
use model::HttpResponse::*;

pub type HandelFn = fn(request:HttpRequest::HttpRequest)->HttpResponse;

#[derive(Debug)]
pub struct HttpHandlerMap{
    handler_map:HashMap<String,HandelFn>
}

impl HttpHandlerMap {
    fn add_handler(&mut self, url_path:String, method:String, function:HandelFn){
        let method = method.to_uppercase();
        let url_method = url_path+"::"+method.as_str();
        println!("- [I] HandlerMap > Added a handler > K: {}", url_method);
        self.handler_map.insert(url_method, function);
    }
    fn handle(&self,request:HttpRequest::HttpRequest)->HttpResponse{
        let k = request.get_url_path_add_methoduppercase();
        println!("Received Request > K: {}",k);
        return match self.handler_map.get(&k) {
            Some(handle) => {
                let response: HttpResponse = handle(request);
                response
            }
            _ => {
                let response = HttpResponse::new()
                    .status_code(404)
                    .result_msg("Not Found".to_string())
                    .context("<h1>404 Not Found</h1>".to_string())
                    .parameters_insert("Content-Type".to_string(), "text/html;charset=utf-8".to_string())
                    .build();
                response
            }
        }
    }
}
