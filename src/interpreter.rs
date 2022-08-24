use crate::nodes::Node;

pub fn interpret(nodes: Vec<Node>, file_name: String){
    let mut file_name: String = file_name[..file_name.len() - 3].to_string().to_string(); // this is so we remove the ".ch"
    file_name.push_str(".c");
    let mut file_data: String = "#include<stdio.h>\n".to_owned();

    for i in nodes{
        //interpret_function(&i, &file_name);
        file_data.push_str(&i.to_c().to_owned());
    }
    std::fs::write(&file_name, file_data).expect("Unable to crate file.");

    std::process::Command::new("gcc").arg(&file_name).spawn().unwrap().wait().expect("Could not compile source code with GCC: make sure GCC is installed on your machine.\n");
    std::fs::remove_file(file_name).expect("Could not delete source file (<filename>.c).\n");
}
