use std::path::PathBuf;
use std::{
    ffi::{OsStr, OsString},
    unimplemented,
};
pub struct VMTranslator {
    path: PathBuf,
    target: PathBuf,
    filename: OsString,
    label_index: u32,
    output: Vec<String>,
}

impl VMTranslator {
    pub fn load(path: PathBuf) -> Self {
        let filename = path.file_name().unwrap().to_os_string();
        let mut target = path.clone();
        let name = path.file_name().unwrap();
        target.push(name);

        VMTranslator {
            path,
            target,
            filename,
            label_index: 1,
            output: vec![],
        }
    }

    pub fn write(&self) {
        let mut asm_path = self.target.clone();
        asm_path.set_extension("asm");
        std::fs::write(asm_path, self.output.join("\n") + "\n").expect("failed to write file");
    }

    pub fn process(&mut self) -> &mut Self {
        let mut deferred = Vec::new();
        for entry in self.path.read_dir().expect("") {
            if let Ok(entry) = entry {
                match entry.file_name().to_str().unwrap() {
                    "Sys.vm" => self.process_single(entry.path()),
                    _ => deferred.push(entry),
                }
            }
        }
        deferred.iter().for_each(|e| {
            self.filename = e.path().file_stem().unwrap().to_os_string();
            self.process_single(e.path())
        });
        self
    }

    fn process_single(&mut self, path: PathBuf) {
        if path.extension() != Some(OsStr::new("vm")) {
            return;
        }
        let vm_code = std::fs::read_to_string(path).expect("cannot read file");
        for raw_line in vm_code.lines() {
            if let Some(before_comment) = raw_line.split("//").next() {
                let line = before_comment.trim();
                if line.is_empty() {
                    continue;
                }
                self.translate_line(line)
            }
        }
    }

    fn emit(&mut self, code: &str) {
        self.output.push(code.to_string())
    }

    fn push_location(&mut self, location: &str) {
        self.emit(&format!(
            "{}\n\
             D=M\n\
             @SP\n\
             A=M\n\
             M=D",
            location
        ));
        self.incr_sp();
    }

    fn translate_line(&mut self, line: &str) {
        let parts: Vec<&str> = line.split(' ').collect();
        // println!("parts: {:?}", parts);
        match (*parts.get(0).unwrap(), parts.get(1), parts.get(2)) {
            ("push", Some(&segment), Some(location)) if location.parse::<u16>().is_ok() => {
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
            ("pop", Some(&segment), Some(location)) if location.parse::<u16>().is_ok() => {
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
            ("call", Some(&function_name), Some(n_args)) => {
                let n = n_args.parse::<u16>().unwrap();
                let return_label = format!("{}$ret.{}", function_name, self.label_index);
                self.label_index += 1;
                self.emit(&format!(
                    "@{}\n\
                     D=A\n\
                     @SP\n\
                     A=M\n\
                     M=D",
                    return_label
                ));
                self.incr_sp();
                self.push_location("@LCL");
                self.push_location("@ARG");
                self.push_location("@THIS");
                self.push_location("@THAT");
                self.emit(&format!(
                    "@{}\n\
                     D=A\n\
                     @SP\n\
                     D=M-D\n\
                     @ARG\n\
                     M=D\n\
                     @SP\n\
                     D=M\n\
                     @LCL\n\
                     M=D\n\
                     @{}\n\
                     0;JMP\n\
                     ({})",
                    n + 5,
                    function_name,
                    return_label
                ));
            }
            ("function", Some(&function_name), Some(n_args)) => {
                self.emit(&format!("({})", function_name,));
                let n = n_args.parse::<u16>().unwrap();
                (0..n).for_each(|_| {
                    self.emit(
                        "@0
                     D=A\n\
                     @SP\n\
                     A=M\n\
                     M=D",
                    );
                    self.incr_sp();
                });
            }
            (move_cmd, Some(&target), None) => match move_cmd {
                "label" => self.emit(&format!("({})", target)),
                "goto" => self.emit(&format!(
                    "@{}\n\
                     0;JMP",
                    target
                )),
                "if-goto" => {
                    self.decr_sp();
                    self.emit(&format!(
                        "@SP\n\
                         A=M\n\
                         D=M\n\
                         @{}\n\
                         D;JNE",
                        target
                    ))
                }
                _ => unimplemented!(),
            },
            (op, None, None) => match op {
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
                "return" => {
                    self.decr_sp();
                    self.emit(&format!(
                        "@LCL\n\
                         D=M\n\
                         @5\n\
                         A=D-A\n\
                         D=M\n\
                         @R13\n\
                         M=D\n\
                         @ARG\n\
                         D=M\n\
                         @SP\n\
                         A=M\n\
                         D=D+M\n\
                         @SP\n\
                         A=M\n\
                         A=D-M\n\
                         MD=D-A\n\
                         @ARG\n\
                         D=M\n\
                         @SP\n\
                         M=D+1\n\
                         @LCL //that\n\
                         A=M-1\n\
                         D=M\n\
                         @THAT\n\
                         M=D\n\
                         @2  //this\n\
                         D=A\n\
                         @LCL\n\
                         A=M-D\n\
                         D=M\n\
                         @THIS\n\
                         M=D\n\
                         @3  //arg\n\
                         D=A\n\
                         @LCL\n\
                         A=M-D\n\
                         D=M\n\
                         @ARG\n\
                         M=D\n\
                         @4  //lcl\n\
                         D=A\n\
                         @LCL\n\
                         A=M-D\n\
                         D=M\n\
                         @LCL\n\
                         M=D"
                    ));

                    // goto ret addr
                    self.emit(&format!(
                        "@R13\n\
                         A=M\n\
                         0;JMP"
                    ));
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
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
        let mut vm_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../projects/");
        vm_path.push(OsStr::new(name));

        let filename = vm_path.file_name().unwrap().to_os_string();
        let mut tst_path = vm_path.clone();
        tst_path.push(&filename);
        tst_path.set_extension("tst");

        VMTranslator::load(vm_path.canonicalize().unwrap())
            .process()
            .write();
        let output = std::process::Command::new("sh")
            .arg(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../tools/CPUEmulator.sh"
            ))
            .arg(tst_path)
            .output()
            .expect("failed to execute process");
        if !output.status.success() {
            println!("error: {:?}", output);
            panic!(output.stderr)
        }
    }

    #[test]
    fn test_simple_add() {
        translate_and_run("07/StackArithmetic/SimpleAdd")
    }
    #[test]
    fn test_stack_test() {
        translate_and_run("07/StackArithmetic/StackTest")
    }
    #[test]
    fn test_basic_test() {
        translate_and_run("07/MemoryAccess/BasicTest")
    }
    #[test]
    fn test_pointer_test() {
        translate_and_run("07/MemoryAccess/PointerTest")
    }
    #[test]
    fn test_static_test() {
        translate_and_run("07/MemoryAccess/StaticTest")
    }

    #[test]
    fn test_basic_loop() {
        translate_and_run("08/ProgramFlow/BasicLoop")
    }

    #[test]
    fn test_fibonacci_series() {
        translate_and_run("08/ProgramFlow/FibonacciSeries")
    }

    #[test]
    fn test_simple_function() {
        translate_and_run("08/FunctionCalls/SimpleFunction")
    }

    #[test]
    fn test_nested_call() {
        translate_and_run("08/FunctionCalls/NestedCall")
    }
    #[test]
    fn test_fibonacci_element() {
        translate_and_run("08/FunctionCalls/FibonacciElement")
    }

    #[test]
    fn test_statics_test() {
        translate_and_run("08/FunctionCalls/StaticsTest")
    }
}
