fn main() {
    match std::env::args().nth(1) {
        Some(cmd) => match cmd.as_str() {
            "assemble" => match std::env::args().nth(2) {
                Some(file) => {
                    let content = std::fs::read_to_string(file.clone()).expect("cannot read file");
                    let mut assembler = assembler::Assembler::new();
                    let out = assembler.process(content);
                    std::fs::write(file.replace(".asm", "-rust.hack"), out)
                        .expect("failed to write file");
                }
                _ => println!("please provide a file"),
            },
            "translate" => match std::env::args().nth(2) {
                Some(file) => {
                    vm_translator::VMTranslator::load(std::path::PathBuf::from(&file))
                        .process()
                        .write();
                }
                _ => println!("please provide a file"),
            },
            _ => println!("no cmd {} is defined", cmd),
        },
        _ => println!("please provide a cmd"),
    }
}
