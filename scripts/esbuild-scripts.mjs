#!/usr/bin/env node
import esbuild from "esbuild";
import { promises as fs } from "fs";

function resolveRelative(filename) {
  return import.meta.resolve(filename).replace(/^file:\/\//, "");
}

const outdir = resolveRelative("../target/ts-scripts");

const args = process.argv.slice(2);
const watch = args.includes("--watch") || args.includes("-w");

const opts = {
  entryPoints: ["./make.ts"].map(resolveRelative),
  sourcemap: false,
  bundle: true,
  platform: "node",
  outdir,
  plugins: [],
  loader: { ".ts": "ts" },
  logLevel: "warning",
};

// clean dist folder
try {
  await fs.rm(outdir, { recursive: true });
} catch (e) {
  // permit no dist folder to begin with
  if (e?.code !== "ENOENT") throw e;
}

if (watch) {
  const ctx = await esbuild.context(opts);
  await ctx.rebuild();
  await ctx.watch();
} else {
  void esbuild.build(opts);
}
