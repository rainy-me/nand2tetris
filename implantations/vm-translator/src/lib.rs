use std::unimplemented;

pub struct VMTranslator {
    file_name: String,
    output: Vec<String>,
}

impl VMTranslator {
    pub fn new() -> Self {
        VMTranslator {
            file_name: "".to_string(),
            output: vec![],
        }
    }

    pub fn process(&mut self, vm_code: String) -> String {
        for raw_line in vm_code.lines() {
            if let Some(before_comment) = raw_line.trim().split("//").next() {
                if before_comment.is_empty() {
                    continue;
                }
                self.translate_line(before_comment)
            }
        }
        self.output.join("\n") + "\n"
    }

    fn translate_line(&mut self, line: &str) {
        let parts: Vec<&str> = line.split(' ').collect();
        match (parts.get(0), parts.get(1), parts.get(2)) {
            (Some(&command), Some(&segment), Some(location)) if location.parse::<u32>().is_ok() => {
                println!("pp: {}+{}+{}", command, segment, location);
                match command {
                    "push" => {
                        if segment == "constant" {
                            self.output.push(format!(
                                "@{}\n\
                                 D=A",
                                location
                            ));
                        } else {
                            self.select_target_addr(segment, location);
                            self.output.push("D=M".to_string());
                        };
                        self.output.push(
                            "@SP\n\
                             A=M\n\
                             M=D"
                            .to_string(),
                        );
                        self.incr_sp();
                    }
                    "pop" => {
                        self.decr_sp();
                        self.output.push(
                            "@SP\n\
                             A=M\n\
                             D=M"
                            .to_string(),
                        );
                        self.select_target_addr(segment, location);
                        self.output.push("M=D".to_string());
                    }
                    _ => {}
                }
            }
            (Some(&op), None, None) => {
                println!("op: {}", op);
                self.select_top_of_stack();
                match op {
                    "add" => self.output.push("M=M+D".to_string()),
                    "sub" => self.output.push("M=M-D".to_string()),
                    "neg" => self.output.push("M=-D".to_string()),
                    "eq" => unimplemented!(),
                    "get" => unimplemented!(),
                    "lt" => unimplemented!(),
                    "and" => unimplemented!(),
                    "or" => unimplemented!(),
                    "not" => unimplemented!(),
                    _ => println!("{}", op),
                }
            }
            _ => {}
        };
    }
    /// make D = y; M = x; SP--
    fn select_top_of_stack(&mut self) {
        self.output.push(
            "@SP\n\
             M=M-1\n\
             A=M\n\
             D=M\n\
             @SP\n\
             A=M-1"
                .to_string(),
        )
    }

    fn select_target_addr<'a>(&mut self, segment: &str, location: &'a str) {
        let update_cmd = match segment {
            "static" => format!("@{}.{}", self.file_name, location),
            "temp" => format!(
                "@5\n\
                 D=A\n\
                 @{}\n\
                 A=D+A",
                location
            ),
            "pointer" => {
                let label = match location {
                    "0" => "THIS",
                    "1" => "THAT",
                    _ => unreachable!(),
                };
                format!("@{}", label)
            }
            _ => {
                let label = match segment {
                    "local" => "LCL",
                    "argument" => "ARG",
                    "this" => "THIS",
                    "that" => "THAT",
                    _ => unreachable!(),
                };
                format!(
                    "@{}\n\
                     D=A\n\
                     @{}\n\
                     A=D+A",
                    location, label
                )
            }
        };
        self.output.push(update_cmd)
    }

    fn incr_sp(&mut self) {
        self.output.push(
            "@SP\n\
             M=M+1"
                .to_string(),
        )
    }
    fn decr_sp(&mut self) {
        self.output.push(
            "@SP\n\
             M=M-1"
                .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::VMTranslator;

    #[test]
    fn translation() {
        let mut vmt = VMTranslator::new();
        vmt.process("pop local 0".to_string());
    }
}
