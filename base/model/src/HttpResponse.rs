use String;
use std::collections::HashMap;
use chrono::prelude::*;

#[derive(Debug)]
pub struct HttpResponse{
    version : String,
    status_code:i32,
    result_msg:String,
    parameters:HashMap<String,String>,
    context:String
}

impl HttpResponse {
    pub fn new() -> HttpResponse {
        let http_date = get_utc_date();
        let mut parameter:HashMap<String,String>= HashMap::new();
        parameter.insert("Date".to_string(),http_date);
        parameter.insert("Connection".to_string(),"close".to_string());
        HttpResponse {
            version: "".to_string(),
            status_code: 0,
            result_msg: "".to_string(),
            parameters: parameter,
            context: "".to_string()
        }
    }
    pub fn context(&mut self,context:String)->&mut Self{
        self.parameters.insert("Content-Length".to_string(), context.len().to_string());
        self.context = context;
        self
    }
    pub fn status_code(&mut self,status_code:i32)->&mut Self{
        self.status_code=status_code;
        self
    }
    pub fn result_msg(&mut self, result_msg: String) ->&mut Self{
        self.result_msg = result_msg;
        self
    }
    pub fn parameters_insert(&mut self,key:String,value:String)->&mut Self{
        self.parameters.insert(key,value);
        self
    }
    pub fn build(&mut self)->HttpResponse{
        HttpResponse {
            version: self.version.clone(),
            status_code: self.status_code.clone(),
            result_msg: self.result_msg.clone(),
            parameters: self.parameters.clone(),
            context: self.context.clone()
        }
    }
}

impl ToString for HttpResponse {
    fn to_string(&self) -> String {
        let a = (&self.version).to_string()+" ";
        let mut response = (&self.version).to_string()+" "+(&self.status_code).to_string().as_str()+" "+(&self.result_msg)+"\r\n";
        for parameter in &self.parameters {
            let item = (parameter.0).to_string()+": "+ (parameter.1.as_str()) +"\r\n";
            response+=item.as_str();
        }
        response+="\r\n";

        response += &self.context;
        if !self.context.is_empty() {
            response+="\r\n";
        }
        response
    }
}

pub trait With{
    fn with(version:String,status_code:i32,result_msg:String,parameters:HashMap<String,String>,context:String)->Self;
}
impl With for HttpResponse {
    fn with(version: String, status_code: i32, result_msg: String, parameters: HashMap<String, String>, context: String) ->Self {
        let http_time = get_utc_date();
        let mut parameter:HashMap<String,String>= parameters;
        parameter.insert("Date".to_string(), http_time);
        parameter.insert("Connection".to_string(),"close".to_string());
        parameter.insert("Content-Length".to_string(), context.len().to_string());
        Self {
            version,
            status_code,
            result_msg,
            parameters: parameter,
            context
        }
    }
}

fn get_utc_date()->String{
    let now:DateTime<Utc> = Utc::now();
    let fmt = "%a, %d %b %Y %H:%M:%S Utc";
    let dft = now.format(fmt);
    let http_date = dft.to_string();
    http_date
}