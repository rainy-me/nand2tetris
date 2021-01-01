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

type Status = "running" | "ok" | "error";
type Result = { status: Status; info: string };

const results: Record<string, Result> = {};

for await (const entry of walk(project)) {
  tryRunTest(entry.path);
}

const watcher = Deno.watchFs(project);

for await (const event of watcher) {
  const [path] = event.paths;
  if (!path) continue;
  tryRunTest(path);
}

function refreshSet(k: string, v: Result) {
  v.info = v.info.trim();
  results[k] = v;
  console.clear();
  Object.entries(results).forEach(([k, v]) => {
    const color =
      v.status === "ok"
        ? (t: string) => bgGreen(black(t))
        : v.status === "running"
        ? cyan
        : bgRed;
    const name = k.padEnd(10);
    const info = color(` ${v.info} `);
    console.log([name, info].join(" : "));
  });
}

async function tryRunTest(path: string) {
  const { name, ext } = extractFileInfo(path);

  if (ext === "asm") {
    const ok = await runCmd(name, [
      "sh",
      "tools/Assembler.sh",
      `projects/${no}/${name}.asm`,
    ]);
    if (!ok) return;

    await runCmd(name, [
      "sh",
      "tools/CPUEmulator.sh",
      `projects/${no}/${name}.tst`,
    ]);
  }

  if (ext === "hdl") {
    await runCmd(name, [
      "sh",
      "tools/HardwareSimulator.sh",
      `projects/${no}/${name}.tst`,
    ]);
  }
}

async function runCmd(name: string, cmd: Deno.RunOptions["cmd"]) {
  refreshSet(name, {
    status: "running",
    info: `running ${cmd.join(" ")}...`,
  });

  const decoder = new TextDecoder();
  const task = Deno.run({
    cmd,
    stdout: "piped",
    stderr: "piped",
  });

  const [ok, error] = await Promise.all(
    [task.output(), task.stderrOutput()].map(async (out) => {
      return decoder.decode(await out);
    })
  );
  task.close();

  const state: Result = error
    ? {
        status: "error",
        info: error,
      }
    : {
        status: "ok",
        info: ok,
      };
  refreshSet(name, state);
  return !error;
}

function extractFileInfo(path: string) {
  const [filename] = path.split("/").reverse();
  const [name, ext] = filename.split(".");
  return {
    name,
    ext,
  };
}
