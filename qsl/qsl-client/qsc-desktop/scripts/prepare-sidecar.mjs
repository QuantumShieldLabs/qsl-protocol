import { chmodSync, copyFileSync, existsSync, mkdirSync } from "node:fs";
import path from "node:path";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const appRoot = path.resolve(scriptDir, "..");
const repoRoot = path.resolve(appRoot, "..", "..", "..");
const resourcesDir = path.join(appRoot, "src-tauri", "resources", "bin");
const qscRelease = path.join(repoRoot, "target", "release", "qsc");

function hostTriple() {
  const output = execFileSync("rustc", ["-vV"], {
    cwd: repoRoot,
    encoding: "utf8"
  });
  const line = output
    .split("\n")
    .find((entry) => entry.startsWith("host: "));
  if (!line) {
    throw new Error("unable to determine rust host triple");
  }
  return line.slice("host: ".length).trim();
}

function main() {
  const triple = hostTriple();
  console.log(`[prepare-sidecar] host=${triple}`);
  execFileSync("cargo", ["build", "--release", "--locked", "-p", "qsc"], {
    cwd: repoRoot,
    stdio: "inherit"
  });
  if (!existsSync(qscRelease)) {
    throw new Error(`missing qsc release binary at ${qscRelease}`);
  }
  mkdirSync(resourcesDir, { recursive: true });
  const targetPath = path.join(resourcesDir, "qsc");
  copyFileSync(qscRelease, targetPath);
  chmodSync(targetPath, 0o755);
  console.log(`[prepare-sidecar] copied ${qscRelease} -> ${targetPath}`);
}

main();
