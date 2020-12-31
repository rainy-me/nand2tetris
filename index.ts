import { walk } from "https://deno.land/std/fs/mod.ts";
import {
  bgRed,
  bgGreen,
  black,
  cyan,
} from " https://deno.land/std/fmt/colors.ts";
import { parse } from "https://deno.land/std/flags/mod.ts";

const { p: no } = parse(Deno.args, { string: ["p"] });
if (!no) {
  console.error("no project is selected");
  Deno.exit();
}

const project = `./projects/${no}`;
console.log(`working on: ${project}`);

const results: Record<string, string> = {};

for await (const entry of walk(project)) {
  tryRunTest(entry.path);
}

const watcher = Deno.watchFs(project);

for await (const event of watcher) {
  const [path] = event.paths;
  if (!path) continue;
  tryRunTest(path);
}

function refreshSet(k: string, v: string) {
  results[k] = v;
  console.clear();
  Object.entries(results).forEach(([k, v]) => {
    const color = v.endsWith("successfully")
      ? (t: string) => bgGreen(black(t))
      : v.endsWith("pending")
      ? cyan
      : bgRed;
    console.log(`${k.padEnd(10)}: ${color(v)}`);
  });
}

async function tryRunTest(path: string) {
  const { name, ext } = extractFileInfo(path);
  if (ext !== "hdl") {
    return;
  }
  refreshSet(name, "pending");
  const cmd = Deno.run({
    cmd: ["sh", "tools/HardwareSimulator.sh", `projects/${no}/${name}.tst`],
    stdout: "piped",
    stderr: "piped",
  });
  const output = await cmd.output();
  const errOutput = await cmd.stderrOutput();
  cmd.close();
  const text = new TextDecoder().decode(output);
  const errText = new TextDecoder().decode(errOutput);
  refreshSet(name, (text || errText).trim());
}

function extractFileInfo(path: string) {
  const [filename] = path.split("/").reverse();
  const [name, ext] = filename.split(".");
  return {
    name,
    ext,
  };
}
