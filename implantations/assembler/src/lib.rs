use std::collections::HashMap;

pub struct Assembler {
    pub symbol_map: HashMap<String, String>,
    pub label_map: HashMap<String, String>,
    r_index: u32,
}

impl Assembler {
    pub fn new() -> Self {
        Assembler {
            symbol_map: HashMap::new(),
            label_map: HashMap::new(),
            r_index: 16,
        }
    }

    pub fn process(&mut self, asm_code: String) -> String {
        let mut out = Vec::new();
        let mut line_index = 0;
        for raw_line in asm_code.lines() {
            if let Some(before_comment) = raw_line.replace(" ", "").split("//").next() {
                let before_comment = before_comment.trim();
                if before_comment.is_empty() || self.extract_label(before_comment, line_index) {
                    continue;
                }
                out.push(before_comment.to_string());
                line_index += 1;
            }
        }
        // println!("{:?}", self.label_map);
        out.iter()
            .map(|line| self.assemble_line(line))
            .collect::<Vec<String>>()
            .join("\n")
            + "\n"
    }

    fn extract_label(&mut self, instruction: &str, index: i32) -> bool {
        if let Some(label) = instruction.strip_prefix("(") {
            self.label_map.insert(
                label[..label.len() - 1].to_string(),
                to_address(&index.to_string()).unwrap(),
            );
            // println!(
            //     "label: {:?}, index: {:?}",
            //     label[..label.len() - 1].to_string(),
            //     index
            // );
            return true;
        }
        false
    }

    fn get_symbol(&mut self, label: &str) -> Option<String> {
        if label.parse::<u32>().is_ok() {
            return to_address(label);
        }
        if let Some(symbol) = get_predefined_symbols(label) {
            return Some(symbol);
        }
        if let Some(symbol) = self.label_map.get(label) {
            return Some(symbol.to_string());
        }
        if let Some(symbol) = self.symbol_map.get(label) {
            return Some(symbol.to_string());
        }

        let this_index = to_address(&self.r_index.to_string());
        self.label_map
            .insert(label.to_string(), this_index.clone().unwrap());
        self.r_index += 1;
        this_index
    }

    fn assemble_line(&mut self, instruction: &str) -> String {
        match instruction.strip_prefix("@") {
            Some(label) => format!("0{}", self.get_symbol(label).unwrap()),
            None => {
                // TODO: this there any better way to do this?
                let parts: Vec<&str> = instruction.split(';').collect();
                let rest = parts.get(0).unwrap_or(&"");
                let jump = parts.get(1).unwrap_or(&"");
                let jump_code = get_jump_code(jump).unwrap_or(&"");
                let parts_2 = rest.split('=').rev().collect::<Vec<&str>>();
                let comp = parts_2.get(0).unwrap_or(&"");
                let dest = parts_2.get(1).unwrap_or(&"");
                let a_indicator_code = if comp.contains('M') { "1" } else { "0" };
                let dest_code = get_dest_code(dest).unwrap_or("");
                let comp_code = get_comp_code(comp).unwrap_or("");
                ["111", a_indicator_code, comp_code, dest_code, jump_code].join("")
            }
        }
    }
}

pub fn get_comp_code(comp_cmd: &str) -> Option<&'static str> {
    match comp_cmd {
        "0" => Some("101010"),
        "1" => Some("111111"),
        "-1" => Some("111010"),
        "D" => Some("001100"),
        "A" => Some("110000"),
        "M" => Some("110000"),
        "!D" => Some("001101"),
        "!A" => Some("110001"),
        "!M" => Some("110001"),
        "-D" => Some("001111"),
        "-A" => Some("110011"),
        "-M" => Some("110011"),
        "D+1" => Some("011111"),
        "A+1" => Some("110111"),
        "M+1" => Some("110111"),
        "D-1" => Some("001110"),
        "A-1" => Some("110010"),
        "M-1" => Some("110010"),
        "D+A" => Some("000010"),
        "D+M" => Some("000010"),
        "D-A" => Some("010011"),
        "D-M" => Some("010011"),
        "A-D" => Some("000111"),
        "M-D" => Some("000111"),
        "D&A" => Some("000000"),
        "D&M" => Some("000000"),
        "D|A" => Some("010101"),
        "D|M" => Some("010101"),
        _ => None,
    }
}

pub fn get_dest_code(dest_cmd: &str) -> Option<&'static str> {
    match dest_cmd {
        "M" => Some("001"),   // RAM[A]
        "D" => Some("010"),   // D register
        "MD" => Some("011"),  // RAM[A] and D register
        "A" => Some("100"),   // A register
        "AM" => Some("101"),  // A register and RAM[A]
        "AD" => Some("110"),  // A register and D register
        "AMD" => Some("111"), // A register, RAM[A], and D register
        "" => Some("000"),    // The value is not stored
        _ => None,
    }
}

pub fn get_jump_code(jump_cmd: &str) -> Option<&'static str> {
    match jump_cmd {
        "JGT" => Some("001"), // if out > 0 jump
        "JEQ" => Some("010"), // if out = 0 jump
        "JGE" => Some("011"), // if out ≥ 0 jump
        "JLT" => Some("100"), // if out < 0 jump
        "JNE" => Some("101"), // if out ≠ 0 jump
        "JLE" => Some("110"), // if out ≤ 0 jump
        "JMP" => Some("111"), // Unconditional jump
        "" => Some("000"),    // no jump
        _ => None,
    }
}

pub fn get_predefined_symbols(label: &str) -> Option<String> {
    // TODO: this could be static simply putting R<N> -> Some(<N>) into match,
    // but I don't know is there any better way that both static and not repeating
    if let Some(register_no) = label.strip_prefix("R") {
        if let Ok(no) = register_no.parse::<u32>() {
            if no <= 15 {
                return to_address(register_no);
            }
        }
    }
    to_address(match label {
        "SP" => "0",
        "LCL" => "1",
        "ARG" => "2",
        "THIS" => "3",
        "THAT" => "4",
        "SCREEN" => "16384",
        "KBD" => "24576",
        _ => "",
    })
}

pub fn to_address(num_like: &str) -> Option<String> {
    match num_like.parse::<i32>() {
        Ok(num) => Some(format!("{:0>15b}", num)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_predefined_symbols, to_address, Assembler};

    #[test]
    fn test_get_predefined_symbols() {
        assert_eq!(
            get_predefined_symbols("SP"),
            Some("000000000000000".to_string())
        );
        assert_eq!(
            get_predefined_symbols("R1"),
            Some("000000000000001".to_string())
        );
        assert_eq!(
            get_predefined_symbols("R2"),
            Some("000000000000010".to_string())
        );
        assert_eq!(get_predefined_symbols("R"), None);
        assert_eq!(get_predefined_symbols("10"), None);
    }
    #[test]
    fn test_to_address() {
        assert_eq!(to_address("1"), Some("000000000000001".to_string()));
        assert_eq!(to_address("2"), Some("000000000000010".to_string()));
    }

    fn compare(name: &str) {
        let base = concat!(env!("CARGO_MANIFEST_DIR"), "/../../projects/06/");
        let asm = std::fs::read_to_string(format!("{}{}.asm", base, name))
            .expect("failed to read test file")
            .to_string();
        let hack = std::fs::read_to_string(format!("{}{}.hack", base, name))
            .expect("failed to read test file")
            .to_string();
        let mut assembler = Assembler::new();
        let out = assembler.process(asm);
        // println!("{}", out);
        // println!("{}", hack);
        assert_eq!(out, hack);
        println!("\tcomparing {} ... ok", name);
    }
    #[test]
    fn test_add() {
        compare("add/Add")
    }
    #[test]
    fn test_max() {
        compare("max/Max")
    }
    #[test]
    fn test_max_l() {
        compare("max/MaxL")
    }
    #[test]
    fn test_rect() {
        compare("rect/Rect")
    }
    #[test]
    fn test_rect_l() {
        compare("rect/RectL")
    }
    #[test]
    fn test_pong() {
        compare("pong/Pong")
    }
    #[test]
    fn test_pong_l() {
        compare("pong/PongL")
    }
}
