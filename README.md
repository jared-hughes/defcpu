# DefCPU

A silly CPU simulator for the goal of simulating assembly solutions on [code.golf](https://code.golf/fibonacci#assembly). The name is based on ([DefAsssembler](https://github.com/NewDefectus/defasm)), which is the assembler for code.golf.

Goals:

- Learn what the instructions do precisely. For example, understand exactly which flags the `div` instruction changes on the code.golf CPU.
- For most code.golf solutions, give the same output as submitting to code.golf would give. For example, `cpuid` should match the code.golf CPU.

Non-goals:

- Support every x86-64 instruction. I intend to only spend time on instructions relevant to code golf holes (x86-64 architecture).
- Get perfect compatability with regards to things like exception handling.

Eventual ideas:

- Debugging facilities to help in golfing.

## Development

We use `clippy` for linting. Run `cargo clippy` or set up your editor to use clippy. The repository includes a `.vscode` that configures rust-analyzer to use clippy.

To run the CLI on a particular file

```sh
cargo run filename.elf
```
