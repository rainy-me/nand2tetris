import { parse } from "https://deno.land/std/flags/mod.ts";

const compCodeMap: Record<string, string> = {
  "0": "101010",
  "1": "111111",
  "-1": "111010",
  D: "001100",
  A: "110000",
  M: "110000",
  "!D": "001101",
  "!A": "110001",
  "!M": "110001",
  "-D": "001111",
  "-A": "110011",
  "-M": "110011",
  "D+1": "011111",
  "A+1": "110111",
  "M+1": "110111",
  "D-1": "001110",
  "A-1": "110010",
  "M-1": "110010",
  "D+A": "000010",
  "D+M": "000010",
  "D-A": "010011",
  "D-M": "010011",
  "A-D": "000111",
  "M-D": "000111",
  "D&A": "000000",
  "D&M": "000000",
  "D|A": "010101",
  "D|M": "010101",
};

const destCodeMap: Record<string, string> = {
  // RAM[A]
  M: "001",
  // D register
  D: "010",
  // RAM[A] and D register
  MD: "011",
  // A register
  A: "100",
  // A register and RAM[A]
  AM: "101",
  // A register and D register
  AD: "110",
  // A register, RAM[A], and D register
  AMD: "111",
  // The value is not stored
  "": "000",
} as const;

const jumpCodeMap: Record<string, string> = {
  // if out > 0 jump
  JGT: "001",
  // if out = 0 jump
  JEQ: "010",
  // if out ≥ 0 jump
  JGE: "011",
  // if out < 0 jump
  JLT: "100",
  // if out ≠ 0 jump
  JNE: "101",
  // if out ≤ 0 jump
  JLE: "110",
  // Unconditional jump
  JMP: "111",
  // no jump
  "": "000",
};

let rIndex = 16;

const RSymbols = Object.fromEntries(
  [...Array(rIndex).keys()].map((k) => [`R${k}`, k.toString()])
);

function getSymbol(label: string) {
  if (/^[0-9]+$/.test(label)) {
    return toAddress(label);
  }
  if (label in predefinedSymbols) {
    return predefinedSymbols[label];
  }
  if (label in labelMap) {
    return labelMap[label];
  }
  if (label in symbolMap) {
    return labelMap[label];
  } else {
    const thisIndex = toAddress(rIndex);
    labelMap[label] = thisIndex;
    rIndex++;
    return thisIndex;
  }
}

const predefinedSymbols: Record<string, string> = Object.fromEntries(
  Object.entries({
    ...RSymbols,
    SP: "0",
    LCL: "1",
    ARG: "2",
    THIS: "3",
    THAT: "4",
    SCREEN: "16384",
    KBD: "24576",
  }).map(([k, v]) => [k, toAddress(v)])
);

const symbolMap: Record<string, string> = {};
const labelMap: Record<string, string> = {};

function toAddress(numLike: string | number) {
  let num: number;
  if (typeof numLike === "string") {
    num = parseInt(numLike, 10);
  } else {
    num = numLike;
  }
  return num.toString(2).padStart(15, "0");
}

function assemble(instruction: string) {
  if (instruction[0] === "@") {
    const label = instruction.slice(1);
    return `0${getSymbol(label)}`;
  } else {
    const [rest, jump = ""] = instruction.split(";");
    const jumpCode = jumpCodeMap[jump];
    const [comp, dest = ""] = rest.split("=").reverse();
    const aIndicatorCode = comp.includes("M") ? "1" : "0";
    const destCode = destCodeMap[dest];
    const compCode = compCodeMap[comp];
    // console.log({ instruction, jump, comp, dest });
    return ["111", aIndicatorCode, compCode, destCode, jumpCode].join("");
  }
}

function extractLabel(instruction: string, index: number): boolean {
  if (instruction.startsWith("(")) {
    const label = instruction.slice(1, instruction.length - 1);
    labelMap[label] = toAddress(index);
    return true;
  }
  return false;
}

async function prepare() {
  const { i: inputFile } = parse(Deno.args, { string: ["i"] });
  if (!inputFile) {
    console.error("no inputFile is provided");
    Deno.exit();
  }
  const out: string[] = [];
  const decoder = new TextDecoder("utf-8");
  const content = await Deno.readFile(inputFile);
  const text = decoder.decode(content);
  let lineIndex = 0;
  text.split("\n").forEach((line) => {
    const [beforeComment] = line.replace(/\s+/g, "").split("//");
    if (!beforeComment) return;
    if (!extractLabel(beforeComment, lineIndex)) {
      out.push(beforeComment);
      lineIndex++;
    }
  });
  const encoder = new TextEncoder();
  Deno.writeFile(
    inputFile.replace(".asm", "-deno.hack"),
    encoder.encode(out.map(assemble).join("\n") + "\n")
  );
}

prepare();
