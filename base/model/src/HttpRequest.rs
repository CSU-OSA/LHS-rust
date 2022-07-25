use std::collections::HashMap;
use super::HttpException::HttpException;

#[derive(Clone, Debug, Default)]
struct HttpRequest {
    method: String,
    url_path: String,
    url_parameters: HashMap<String, String>,
    version: String,
    parameters: HashMap<String, String>,
    context: String,
}



impl HttpRequest {
    pub fn new(http_msg_str: &[char]) -> Result<HttpRequest, HttpException> {
        let mut http_request = HttpRequest::default();
        let mut parse_status: bool = false;
        let mut p = 0;
        let mut p_in = 0;
        let mut line_buff: [char; 1024] = ['\0'; 1024];
        while http_msg_str[p] != '\0' {
            if http_msg_str[p] == '\r' {
                if p + 1 <= 512 && http_msg_str[p + 1] == '\n' {
                    line_buff[p_in] = '\0';
                    p = p + 2;
                    //读完一行请求报文，解析
                    if parse_status == false {
                        //第一行请求
                        if parse_http_head_request_line(&line_buff, &mut http_request) == false {
                            return Err(HttpException::from("Exception when parsing first line."));
                        }
                        p_in = 0;
                        parse_status = true;
                    } else if parse_status == true {
                        let ret = parse_http_head_request_params(&line_buff, &mut http_request);
                        if ret == -1 {
                            return Err(HttpException::from("Exception when parsing request params."));
                        }
                        continue;
                    } else {
                        return Err(HttpException::from("Exception when reading a line."));
                    }
                }
                line_buff[p_in] = http_msg_str[p];
                p_in += 1;
                p += 1;
            }
            let mut context = String::new();
            while http_msg_str[p] != '\0' {
                context = context + http_msg_str[p].to_string().as_str();
            }
            http_request.context(context);
        }
        Ok(http_request.build())
    }
    //build模式初始化
    fn default() -> Self {
        Self {
            method: String::from(""),
            url_path: String::from(""),
            url_parameters: Default::default(),
            version: String::from(""),
            parameters: Default::default(),
            context: String::from(""),
        }
    }
    fn method(&mut self, value: String) -> &mut Self {
        self.method = value;
        self
    }
    fn url_path(&mut self, value: String) -> &mut HttpRequest {
        self.url_path = value;
        self
    }
    fn url_parameters(&mut self, value: HashMap<String, String>) -> &mut HttpRequest {
        self.url_parameters = value;
        self
    }
    fn url_parameters_insert(&mut self, key: String, value: String) {
        self.url_parameters.insert(key, value);
    }
    fn version(&mut self, value: String) -> &mut HttpRequest {
        self.version = value;
        self
    }
    fn parameters(&mut self, value: HashMap<String, String>) -> &mut HttpRequest {
        self.parameters = value;
        self
    }
    fn context(&mut self, value: String) -> &mut HttpRequest {
        self.context = String::from(value);
        self
    }
    pub fn build(self) -> HttpRequest {
        HttpRequest {
            method: self.method,
            url_path: self.url_path,
            url_parameters: self.url_parameters,
            version: self.version,
            parameters: self.parameters,
            context: self.context,
        }
    }
}


fn parse_http_head_request_line(a_line: &[char], http_req: &mut HttpRequest) -> bool {
    let mut p = 0;
    let mut count = 0;

    //读method
    let mut method = String::new();
    while a_line[p] != '\0' {
        match a_line[p] {
            ' ' => {
                count += 1;
                p += 1;
                break;
            }
            _ => {
                method = method + a_line[p].to_string().as_str();
                p += 1;
            }
        }
    }

    http_req.method(method);
    //读url_path
    let mut url_path = String::new();
    while a_line[p] != '\0' {
        //发现'?'进入Url参数读取
        match a_line[p] {
            '?' => {
                count += 1;
                p += 1;
                break;
            }
            ' ' => {
                count += 1;
                break;
            }
            _ => {
                url_path = url_path + a_line[p].to_string().as_str();
                p += 1;
            }
        }
    }

    http_req.url_path(url_path);
    //读url_params

    let mut url_parameters: HashMap<String, String> = HashMap::new();
    while a_line[p] != '\0' {
        let (mut k, mut v) = (String::new(), String::new());
        match a_line[p] {
            ' ' => {
                p += 1;
                break;
            }
            _ => {
                k.clear();
                v.clear();
                while a_line[p] != '\0' && a_line[p] != '=' {
                    k = k + a_line[p].to_string().as_str();
                    p += 1;
                }
                p += 1;
                while a_line[p] != '\0' && a_line[p] != '&' && a_line[p] != ' ' {
                    v = v + a_line[p].to_string().as_str();
                    p += 1;
                }
                if k.is_empty() || v.is_empty() {
                    return false;
                }
                url_parameters.insert(k, v);
            }
        }
    }
    http_req.url_parameters(url_parameters);
    //读version
    let mut version = String::new();
    while a_line[p] != '\0' {
        match a_line[p] {
            ' ' => {
                count += 1;
                break;
            }
            _ => {
                version = version + a_line[p].to_string().as_str();
                p += 1;
            }
        }
    }
    if count != 2 {
        return false;
    }
    true
}

fn parse_http_head_request_params(a_line: &[char], http_req: &mut HttpRequest) -> i8 {
    let mut p = 0;//总指针
    let mut p_in = 0;//请求参数str内指针
    let mut parts: [[char; 64]; 2] = [[' '; 64], [' '; 64]];//请求参数str组
    //逐字处理（K部分）
    while a_line[p] != '\0' {
        match a_line[p] {
            //遇到第一个冒号，跳过两个字符，进入Value部分
            ':' => {
                parts[0][p_in] = '\0';
                p_in = 0;
                p += 2;
                break;
            }
            _ => {
                parts[0][p_in] = a_line[p];
                p_in += 1;
                p += 1;
            }
        }
    }
    //逐字处理（V部分）
    while a_line[p] != '\0' {
        parts[1][p_in] = a_line[p];
        p_in += 1;
        p += 1;
    }
    let (k, v) = (parts[0].iter().collect::<String>(), parts[1].iter().collect::<String>());
    if k.is_empty() && v.is_empty() {
        return 1;
    } else if k.is_empty() || v.is_empty() {
        return -1;
    }
    http_req.url_parameters_insert(k, v);
    0
}
