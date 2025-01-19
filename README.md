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

## Installation

- Run `npm install` for JS dependencies.
- Install `wasm-pack` with the the [wasm-pack installer](https://rustwasm.github.io/wasm-pack/installer/).
- Nothing for Rust dependencies -- it installs automatically.

## List of scripts:

- `./make lint`: Lint (Rust is linted with `cargo clippy` and Typescript with `tsc`).
- `./make build`: Build everything else. Pass `./make build -w` to fix.
- `./make test`: Test (It's currently an amalgamation of bash scripts).
- `./build-site.sh`: Build site (TODO: auto-rebuild).
  - This generated static files in the `public-deploy/` folder, including wasm builds of the Rust code.
  - To serve locally, I run (TODO: make this proper, and hook to a watch server)
    ```sh
    ./build-site.sh && npx http-server public-deploy/ -c-1
    ```
- `cargo run -- run file.elf --random-seed 123456`: Interpret an ELF file
- `cargo run -- dis file.elf`: Disassemble an ELF file

## Recommended editor plugins:

- Something for Rust. VS Code: `rust-lang.rust-analyzer`
  - The repository includes a `.vscode` that configures rust-analyzer to use clippy.
- ShellCheck. VS Code: `timonwong.shellcheck` (TODO: Include shellcheck in lint script)
- Something for Typescript. VS Code: I think it's built-in.
  - Make sure it's configured to use the workspace Typescript version. When a TypeScript file is open in the editor, run (Ctrl+Shift+P) the command "Typescript: Select TypeScript Version"

## References and tooling

My primary reference is the October 2024 version of [Intel® 64 and IA-32 Architectures Software Developer's Manual.](https://software.intel.com/en-us/download/intel-64-and-ia-32-architectures-sdm-combined-volumes-1-2a-2b-2c-2d-3a-3b-3c-3d-and-4). Unqualified references in comments to "Vol 1", "Vol 2A", etc. refer to this manual.

The November 18, 2024 version of the [System V Application Binary Interface](https://gitlab.com/x86-psABIs/x86-64-ABI) was helpful for working out the initial process stack, along with the following linux kernel source files:

- `fs/binfmt_elf.c`
- `fs/exec.c`
- `arch/x86/kernel/process.c`

[Felix Cloutier's Insstruction Reference](https://www.felixcloutier.com/x86) is good to save a bunch of jumping around in the PDF.

The [coder64](http://ref.x86asm.net/coder64.html) and [geek64](http://ref.x86asm.net/geek64.html) references of MazeGen's X86 Opcode and Instruction Reference are quite thorough. The [index](http://ref.x86asm.net/index.html) helps describe how to read them.

`objdump` is great for giving the segments etc. of an ELF file with the `-x` flag, but it doesn't seem to disassemble (`-d`) the ELFs produced by DefAssembler. `readelf` works about as well as `objdump`.

For quick things, https://defuse.ca/online-x86-assembler.htm is good for assembly/disassembling small bits of asm. Make sure to switch to x64 architecture.

`gdb` can show disassembly with the `disassemble` command. For example, `disassemble /m 0x400000, +0x49` disassembles the 73 bytes starting at `0x400000`.

`nm` is irrelevant because we tend to not have any symbols to deal with.

## Yap about the testing

### Integration Tests

The script `build-elfs.sh` uses DefAssembler to make ELF files from the x86 asm in `sources/*.s`. These are NOT checked into git because there's no point. DefAssembler is fast enough, and they're binary files.

The script `run-sources.sh` uses the code.golf servers to run the assembly and put the outputs in the `expected` directory. Out of respect for the servers, these are checked into git and cached using the `sha256sum` of the sources as the sole cache key.

### Disassembly Tests

The script `gen-fuzz.sh` generates some of the files in `tests/disassembly/sources` which are essentially just fuzz tests. A bunch of random bytes from a given pool.

The script `validate-sources.sh` takes the hexadecimal bytes on the left of each file in `tests/disassembly/sources`, puts them in an ELF file (with defasm), then disassembles them with `gdb` (I couldn't get `objdump` to work). If it is ran as `validate-sources.sh apply` (which is automatic from `./make.sh apply`), then the sources get overwritten with the disassembly, so the script essentially behaves as ensuring the disassembly on the right is up-to-date with the hex source on the left.

The script `test-disassembly.sh` is automated testing. It hooks into `defcpu dis` to verify that DefCPU gives the same disassembly as `gdb`.
