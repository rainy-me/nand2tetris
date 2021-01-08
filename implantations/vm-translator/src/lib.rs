pub struct VMTranslator {
    file_name: String,
    label_index: u32,
    output: Vec<String>,
}

impl VMTranslator {
    pub fn new() -> Self {
        VMTranslator {
            file_name: "".to_string(),
            label_index: 1,
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
                        self.select_target_addr(segment, location);
                        self.output.push(
                            "D=A\n\
                             @SP\n\
                             A=M\n\
                             D=D+M\n\
                             @SP\n\
                             A=M\n\
                             A=D-M\n\
                             M=D-A"
                                .to_string(),
                        );
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
                    "eq" => {
                        self.output.push("D=M-D".to_string());
                        self.emit_logical_commands("JEQ");
                    }
                    "get" => {
                        self.output.push("D=M-D".to_string());
                        self.emit_logical_commands("JGT");
                    }
                    "lt" => {
                        self.output.push("D=M-D".to_string());
                        self.emit_logical_commands("JLT");
                    }
                    "and" => {
                        self.output.push("D=M&D".to_string());
                        self.emit_logical_commands("JNE");
                    }
                    "or" => {
                        self.output.push("D=M|D".to_string());
                        self.emit_logical_commands("JNE");
                    }
                    "not" => {
                        self.output.push("D=M".to_string());
                        self.emit_logical_commands("JEQ");
                    }
                    _ => println!("{}", op),
                }
            }
            _ => {}
        };
    }

    fn emit_logical_commands(&mut self, condition: &str) {
        self.output.push(format!(
            "@IF_{0}\n\
             D;{1}\n\
             @ELSE_{0}\n\
             0;JMP\n\
             (IF_{0})\n\
                 @SP\n\
                 A=M-1\n\
                 M=-1\n\
                 @END_{0}\n\
                 0;JMP\n\
             (ELSE_{0})\n\
                 @SP\n\
                 A=M-1\n\
                 M=0\n\
             (END_{0})",
            self.label_index, condition
        ));
        self.label_index += 1;
    }

    /// make M = x; D = y; SP--
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
