use clap::Command;

pub struct CLIInput {
    pub input_path: String,
    pub input_name: String,
    pub output_path: String,
    pub print_tokens: bool,
    pub print_ast: bool,
    pub verbose: u32,
}

pub fn init_cli() -> CLIInput{
    let matches = Command::new("yotc").get_matches();

    return CLIInput{
        input_path: "".to_string(),
        input_name: "".to_string(),
        output_path: "".to_string(),
        print_tokens: true,
        print_ast: true,
        verbose: 1,

    }
}
