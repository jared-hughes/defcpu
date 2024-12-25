# DefCPU

A silly CPU emulator (interpreter) for the goal of simulating assembly solutions on [code.golf](https://code.golf/fibonacci#assembly). The name is based on ([DefAsssembler](https://github.com/NewDefectus/defasm)), which is the assembler for code.golf x86 assembly solutions.

Goals:

- Learn what the instructions do precisely. For example, understand exactly which flags the `div` instruction changes on the code.golf CPU.
- For most code.golf solutions, give the same output as submitting to code.golf would give. For example, `cpuid` should match the code.golf CPU.

Non-goals:

- Support every x86-64 instruction. I intend to only spend time on instructions relevant to code golf holes (x86-64 architecture).
- Get perfect compatability with regards to things like exception handling.
- Great performance. While I don't want it to too sluggish, I wouldn't be surprised if the performance ends up hundreds of times slower than running natively.

Eventual ideas:

- Debugging facilities to help in golfing.

## Development

We use `clippy` for linting. Run `cargo clippy` or set up your editor to use clippy. The repository includes a `.vscode` that configures rust-analyzer to use clippy.

To run the CLI on a particular file

```sh
cargo run filename.elf
```

## Tests

To regenerate test files, first make sure defasm and golfc are installed by running `npm install`.

The script `./build-elfs.sh` uses DefAssembler to make ELF files from the x86 asm in `sources/*.s`. These are NOT checked into git because there's no point. DefAssembler is fast enough, and they're binary files.

The script `./run-sources.sh` uses the code.golf servers to run the assembly and put the outputs in the `outputs` directory. Out of respect for the servers, these are checked into git and cached using the `sha256sum` of the sources as the sole cache key.
