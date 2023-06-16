use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use http_server::ThreadPool;

fn main() {
    const ADDRESS: &str = "localhost:7878";
    const THREADPOOL_SIZE:usize = 10;

    let listener = TcpListener::bind(ADDRESS).unwrap();
    println!("Server listening on : {ADDRESS}");

    let thread_pool = ThreadPool::new(THREADPOOL_SIZE);

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        thread_pool.execute(
            ||{
                handle_connection(stream);
            }
        );


    }
}

fn handle_connection(mut stream:TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let http_reqs:Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line|!line.is_empty())
        .collect();


    print_lines(&http_reqs);

    /*if http_reqs.get(0).unwrap() == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("pages/index.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("pages/404.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();

    }*/
    //refactor ↓↓↓
    let (status_line,filename) = if http_reqs.get(0).unwrap() == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    }else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let filename = format!("pages/{filename}");
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();


}

fn print_lines(http_reqs:&Vec<String>){
    println!("_____________________________");
    for _content in http_reqs{
        println!("\t{_content}");
    }
}