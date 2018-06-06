extern crate fizzbuzz;
extern crate iron;

use fizzbuzz::fizzbuzz_number_formatter::FizzbuzzMessageFormatter;

use iron::IronResult;

use iron::headers::*;
use iron::method::Method;
use iron::mime::*;
use iron::prelude::Iron;
use iron::request::Request;
use iron::response::Response;
use iron::status;

fn main() {
    Iron::new(process_request).http("localhost:3000").unwrap();
}

fn process_request(request: &mut Request) -> IronResult<Response>  {

    let requested_content_type = determine_mime_type(request.headers.get::<Accept>().unwrap());

    let response = if request.method != Method::Get {
        Ok(Response::with((status::MethodNotAllowed, "Method not allowed")))
    } else if requested_content_type == None {
        Ok(Response::with((status::NotAcceptable, "Not acceptable")))
    } else {
        prepare_fizzbuzz_response(request, &requested_content_type.unwrap())
    };

    response
}

fn prepare_fizzbuzz_response(request: &Request, requested_content_type: &ContentType) -> IronResult<Response> {

    let number = request.url.path()[0].parse::<i64>().unwrap();

    let content = if requested_content_type == &ContentType::json() {
        generate_json_content(number)
    } else {
        generate_plaintext_content(number)
    };

    let mut response = Response::with((status::Ok, content));
    response.headers.set(requested_content_type.to_owned());

    Ok(response)
}

fn determine_mime_type(accept: &Accept) -> Option<ContentType> {

    let json: Mime = "application/json".parse().unwrap();
    let text: Mime = "text/plain".parse().unwrap();

    let mut requested_content_type = None;

    for content_type in accept.iter() {
        if content_type.item == json {
            requested_content_type = Some(ContentType::json());
        } else if content_type.item == text {
            requested_content_type = Some(ContentType::plaintext());
        }
    }

    requested_content_type
}

fn generate_json_content(number : i64) -> String {
    let mut response = "{\"fizzbuzz\" : \"".to_string();
    response.push_str(&FizzbuzzMessageFormatter::default().format_number(number));
    response.push_str("\",\"number\" : \"");
    response.push_str(&number.to_string());
    response.push_str("\"}");
    response
}

fn generate_plaintext_content(number : i64) -> String {
    FizzbuzzMessageFormatter::default().format_number(number)
}
