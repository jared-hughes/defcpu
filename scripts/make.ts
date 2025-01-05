import child_process, { SpawnOptionsWithoutStdio } from "node:child_process";
import yargs from "yargs/yargs";

// Assumes it is called from the project directory.
yargs(process.argv.slice(2))
  .usage("$0 command")
  .command(
    "lint",
    "Lint Rust and Typescript code",
    (yargs) => yargs,
    async (_argv) => {
      await cmdLint();
    }
  )
  .command(
    "build",
    "Build stuff",
    (yargs) =>
      yargs.option("w", {
        type: "boolean",
        default: false,
        alias: "write",
        describe: "Overwrite sources in tests/disassembly/sources when needed.",
      }),
    async (argv) => {
      await cmdBuild(argv.w);
    }
  )
  .command(
    "test",
    "Test everything",
    (yargs) => yargs,
    async (_argv) => {
      await cmdTest();
    }
  )
  .strict()
  .demandCommand()
  .help()
  .parse();

async function cmdLint() {
  await exec("cargo", ["clippy"], {});
  await exec("./node_modules/.bin/tsc", ["--build"], {});
}

async function cmdBuild(write: boolean) {
  await exec("./validate-sources.sh", write ? ["-w"] : [], {
    cwd: "./tests/disassembly",
  });
  await exec("./run-sources.sh", [], { cwd: "./tests/integration" });
  await exec("./build-elfs.sh", [], { cwd: "./tests/integration" });
}

async function cmdTest() {
  await cmdBuild(false);
  await exec("cargo", ["build", "--release"], {});
  await exec("cargo", ["test"], {});
  await exec("./test-disassembly.sh", [], {
    cwd: "./tests/disassembly",
  });
  await exec("./test-run.sh", [], {
    cwd: "./tests/integration",
  });
}

async function exec(
  command: string,
  args: readonly string[],
  options: SpawnOptionsWithoutStdio
) {
  const dbg = `'${command}' ${JSON.stringify(args)}`;
  console.log("Spawning", dbg);

  return new Promise<void>((resolve) => {
    const ps = child_process.spawn(command, args, {
      ...options,
      stdio: "inherit",
    });

    ps.on("close", (code) => {
      if (code !== 0) {
        console.error(`Script failed: code ${code} for ${dbg}.`);
        process.exit(code);
      }
      resolve();
    });

    ps.on("error", (err) => {
      console.error(
        `Failed to start subprocess for ${dbg}`,
        typeof err === "object" && err != null ? (err as any).code : ""
      );
      process.exit(1);
    });
  });
}
