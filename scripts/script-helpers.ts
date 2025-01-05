import child_process, { SpawnOptionsWithoutStdio } from "child_process";

interface Opts extends SpawnOptionsWithoutStdio {
  capture_stdout?: boolean;
  allowNonzeroExit?: boolean;
  quiet?: boolean;
}

interface ExecOut {
  stdout: Buffer;
}

export async function execStdout(
  command: string,
  args: readonly string[],
  opts?: Exclude<Opts, "capture_stdout">
) {
  return (
    await exec(command, args, { quiet: true, ...opts, capture_stdout: true })
  ).stdout;
}

export async function exec(
  command: string,
  args: readonly string[],
  opts?: Opts
) {
  const dbg = `'${command}' ${JSON.stringify(args)}`;
  if (!opts?.quiet) console.log("Spawning", dbg);

  return new Promise<ExecOut>((resolve) => {
    const ps = child_process.spawn(command, args, {
      stdio: [
        process.stdin,
        opts?.capture_stdout ? "pipe" : process.stdout,
        process.stderr,
      ],
      ...opts,
    });

    let stdoutChunks: Buffer[] = [];
    if (opts?.capture_stdout) {
      ps.stdout?.on("data", (chunk: Buffer) => {
        stdoutChunks.push(chunk);
      });
    }

    ps.on("close", (code) => {
      if (!opts?.allowNonzeroExit && code !== 0) {
        console.error(`Script failed: code ${code} for ${dbg}.`);
        process.exit(code);
      }
      resolve({
        stdout: Buffer.concat(stdoutChunks),
      });
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

export async function sha256sum(filename: string) {
  const { stdout } = await exec("sha256sum", [filename], {
    capture_stdout: true,
    quiet: true,
  });
  if (!stdout) fail("Failed when piping out of sha256sum.");
  return stdout;
}

export function fail(s: string): never {
  console.error(s);
  process.exit(1);
}
