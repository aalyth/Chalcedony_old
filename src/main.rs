mod lexer;

fn main() {
    /*
    let arguments: Vec<String> = std::env::args().collect();
    let file_name: &str = &arguments[1];
    let source_code = std::fs::read_to_string(file_name).expect("File not found!");
    */
    let source_code = std::fs::read_to_string("1.ch").expect("File not found!");
    let tokens = lexer::lexer(&source_code);
    for i in tokens{
        println!("{:?}", i);
    }
}
