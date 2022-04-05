use std::collections::HashMap;
use std::io::{Result,Write};


//import three traits

#[derive(Debug, PartialEq, Clone)]


//define a struct that includes some properties
pub struct HttpResponse<'a> {
     version: &'a str,
     status_code: &'a str,
     status_text: &'a str,
     headers: Option<HashMap<&'a str,&'a str>>,
     body:Option<String>, 
}

//implement trait for struct above
//it can specify the default value to HttpResponse(because the trait has a method related to struct(HttpResponse))

impl <'a> Default for HttpResponse<'a> {
     fn default()->Self {

          //some default values

          Self {
               version: "HTTP/1.1".into(),
               status_code:"200".into(),
               status_text:"OK".into(),
               headers:None,
               body:None,
          }
     }
}

//implement methods

impl<'a> HttpResponse<'a> {

     //create new struct (HttpResponse)
     pub fn new (
     
          status_code: &'a str,
          headers: Option<HashMap<&'a str,&'a str>>,
          body: Option<String>

     ) -> HttpResponse<'a> {

          let mut response: HttpResponse<'a> = HttpResponse::default();
          if status_code != "200" {
               response.status_code = status_code.into();
          }; 
          response.headers = match &headers {
              Some(_h) => headers,
              None => {
                   let mut h = HashMap::new();
                   h.insert("Content-Type","text/html");
                   Some(h)
              }
          };
          response.status_text = match response.status_code {
               "200" => "OK".into(),
               "400" =>"Bad Request".into(),
               "404" =>"Not Found".into(),
               "500" => "Internal Server Error".into(),
               _ =>"Not Found".into(),
          };
          response.body = body;

          response
     }

     //send a response
     pub fn send_response(&self, write_stream: &mut impl Write)-> Result<()> {

          let res = self.clone();
          let response_string: String = String::from(res);
          let _ = write!(write_stream, "{}",response_string);

          Ok(())
     }
     
     fn version(&self) -> &str {
          self.version
     }

     fn status_code(&self)->&str {
          self.status_code
          }

     fn status_text(&self)->&str {
          self.status_text
     }

     fn headers(&self) -> String {

          let map: HashMap<&str,&str> = self.headers.clone().unwrap();
          let mut header_string :String = "".into();

          for (k,v) in map.iter(){
               header_string = format!("{}{}:{}\r\n", header_string, k,v);
          }
          header_string
     }

     pub fn body(&self) -> &str {
          match &self.body {
               Some(b) => b.as_str(),
               None => "",
          }
     }
}

//implement trait for struct above
//convert HttpResponse to String

impl <'a> From<HttpResponse<'a>> for String {
     fn from(res:HttpResponse)->String {
          let res1 = res.clone();
          format!(
               "{} {} {}\r\n{}Content-Length:{}\r\n\r\n{}",
               &res1.version(),
               &res1.status_code(),
               &res1.status_text(),
               &res1.headers(),
               &res.body.unwrap().len(),
               &res1.body()
          )
     }
}

//test method

#[cfg(test)]

mod tests {

     use super::*;

     #[test]

     fn test_response_struct_creation_404() {
          let response_actual = HttpResponse::new(
               "404", 
               None, 
               Some("xxxx".into())
          );
      
          let response_expected = HttpResponse{
               version: "HTTP/1.1",
               status_code:"404",
               status_text:"Not Found",
               headers: {
                    let mut h = HashMap::new();
                    h.insert("Content-Type","text/html");
                    Some(h)
               },
               body:Some("xxxx".into())
          };

          assert_eq!(response_expected,response_actual);
          
     }

     #[test]

     fn test_response_struct_creation_200() {
          let response_actual = HttpResponse::new("200",None,Some("xxxx".into()));
      
          let response_expected = HttpResponse{
               version: "HTTP/1.1",
               status_code:"200",
               status_text:"OK",
               headers: {
                    let mut h = HashMap::new();
                    h.insert("Content-Type","text/html");
                    Some(h)
               },
              body:Some("xxxx".into()),
          };

          assert_eq!(response_expected,response_actual);
          
     }

     #[test]

     fn test_http_response_creation() {
          let response_expected = HttpResponse {
               version: "HTTP/1.1",
               status_code: "404",
               status_text: "Not Found",
               headers:{
                    let mut h = HashMap::new();
                    h.insert("Content-Type","text/html");
                    Some(h)
               },
               body:Some("xxxx".into()),
          };
          let http_string: String = response_expected.into();
          let actual_string = "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length:4\r\n\r\nxxxx";

          assert_eq!(http_string,actual_string);
     }

}