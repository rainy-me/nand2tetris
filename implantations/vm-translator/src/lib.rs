use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
pub struct VMTranslator {
    path: PathBuf,
    filename: OsString,
    label_index: u32,
    output: Vec<String>,
}

impl VMTranslator {
    pub fn load(path: &str) -> Self {
        let path = PathBuf::from(path);
        let filename = path.file_stem().unwrap().to_os_string();
        VMTranslator {
            path,
            filename,
            label_index: 1,
            output: vec![],
        }
    }

    pub fn process(&mut self) -> &mut Self {
        let vm_code = std::fs::read_to_string(&self.path).expect("cannot read file");
        for raw_line in vm_code.lines() {
            if let Some(before_comment) = raw_line.trim().split("//").next() {
                if before_comment.is_empty() {
                    continue;
                }
                self.translate_line(before_comment)
            }
        }
        self
    }

    pub fn write(&self) {
        let mut target_file = self.path.clone();
        target_file.set_extension(OsStr::new("asm"));
        std::fs::write(target_file, self.output.join("\n") + "\n").expect("failed to write file");
    }

    fn emit(&mut self, code: &str) {
        self.output.push(code.to_string())
    }

    fn translate_line(&mut self, line: &str) {
        let parts: Vec<&str> = line.split(' ').collect();
        match (parts.get(0), parts.get(1), parts.get(2)) {
            (Some(&command), Some(&segment), Some(location)) if location.parse::<u32>().is_ok() => {
                // println!("pp: {}+{}+{}", command, segment, location);
                match command {
                    "push" => {
                        if segment == "constant" {
                            self.emit(&format!(
                                "@{}\n\
                                 D=A",
                                location
                            ));
                        } else {
                            self.select_target_addr(segment, location);
                            self.emit("D=M");
                        };
                        self.emit(
                            "@SP\n\
                                  A=M\n\
                                  M=D",
                        );
                        self.incr_sp();
                    }
                    "pop" => {
                        self.decr_sp();
                        self.select_target_addr(segment, location);
                        self.emit(
                            "D=A\n\
                                  @SP\n\
                                  A=M\n\
                                  D=D+M\n\
                                  @SP\n\
                                  A=M\n\
                                  A=D-M\n\
                                  M=D-A",
                        );
                    }
                    _ => {}
                }
            }
            (Some(&op), None, None) => match op {
                "add" => self.operate_top_two("M=M+D"),
                "sub" => self.operate_top_two("M=M-D"),
                "neg" => self.operate_top("M=-M"),
                "eq" => {
                    self.operate_top_two("D=M-D");
                    self.emit_logical_commands("JEQ");
                }
                "gt" => {
                    self.operate_top_two("D=M-D");
                    self.emit_logical_commands("JGT");
                }
                "lt" => {
                    self.operate_top_two("D=M-D");
                    self.emit_logical_commands("JLT");
                }
                "and" => self.operate_top_two("M=M&D"),
                "or" => self.operate_top_two("M=M|D"),
                "not" => self.operate_top("M=!M"),
                _ => println!("{}", op),
            },
            _ => {}
        };
    }

    fn select_target_addr<'a>(&mut self, segment: &str, location: &'a str) {
        let update_cmd = match segment {
            "static" => format!("@{}.{}", self.filename.to_string_lossy(), location),
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
                     A=D+M",
                    location, label
                )
            }
        };
        self.emit(&update_cmd)
    }

    fn emit_logical_commands(&mut self, condition: &str) {
        self.emit(&format!(
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
    fn operate_top_two(&mut self, op_code: &str) {
        self.emit(
            "@SP\n\
                  M=M-1\n\
                  A=M\n\
                  D=M\n\
                  @SP\n\
                  A=M-1",
        );
        self.emit(op_code)
    }

    fn operate_top(&mut self, op_code: &str) {
        self.emit("@SP\nA=M-1");
        self.emit(op_code)
    }

    fn incr_sp(&mut self) {
        self.emit(
            "@SP\n\
                  M=M+1",
        )
    }
    fn decr_sp(&mut self) {
        self.emit(
            "@SP\n\
                  M=M-1",
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::VMTranslator;
    use std::ffi::OsStr;
    use std::path::PathBuf;

    fn translate_and_run(name: &str) {
        let mut vm_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../projects/07/");
        vm_path.push(OsStr::new(name));
        vm_path.set_extension("vm");
        let mut tst_path = vm_path.clone();
        tst_path.set_extension("tst");
        VMTranslator::load(&vm_path.to_string_lossy())
            .process()
            .write();
        println!("\ttranslating {} ... ok", name);
        let status = std::process::Command::new("sh")
            .arg(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../tools/CPUEmulator.sh"
            ))
            .arg(tst_path)
            .status()
            .expect("failed to execute process");
        println!("\ttesting result ... {}", status);
    }

    #[test]
    fn test_simple_add() {
        translate_and_run("StackArithmetic/SimpleAdd/SimpleAdd")
    }
    #[test]
    fn test_stack_test() {
        translate_and_run("StackArithmetic/StackTest/StackTest")
    }
    #[test]
    fn test_basic_test() {
        translate_and_run("MemoryAccess/BasicTest/BasicTest")
    }
    #[test]
    fn test_pointer_test() {
        translate_and_run("MemoryAccess/PointerTest/PointerTest")
    }
    #[test]
    fn test_static_test() {
        translate_and_run("MemoryAccess/StaticTest/StaticTest")
    }
}
