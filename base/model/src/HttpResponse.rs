use String;
use std::collections::HashMap;

struct HttpResponse{
    version : String,
    status_code:i32,
    result_msg:String,
    parameters:HashMap<String,String>,
    context:String
}

impl HttpResponse {

}