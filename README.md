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
cargo run -- run filename.elf
```

To get a disassembly:

```sh
cargo run -- dis filename.elf
```

My primary reference is the October 2024 version of [Intel® 64 and IA-32 Architectures Software Developer's Manual.](https://software.intel.com/en-us/download/intel-64-and-ia-32-architectures-sdm-combined-volumes-1-2a-2b-2c-2d-3a-3b-3c-3d-and-4). Unqualified references in comments to "Vol 1", "Vol 2A", etc. refer to this manual.

## Tests

To regenerate test files, first make sure defasm and golfc are installed by running `npm install`.

The script `./build-elfs.sh` uses DefAssembler to make ELF files from the x86 asm in `sources/*.s`. These are NOT checked into git because there's no point. DefAssembler is fast enough, and they're binary files.

The script `./run-sources.sh` uses the code.golf servers to run the assembly and put the outputs in the `outputs` directory. Out of respect for the servers, these are checked into git and cached using the `sha256sum` of the sources as the sole cache key.

## Future correctness pass notes

- `REX.W` prefix switching instructions.
- Any `REX` prefix switching legacy to 64-bit mode instructions?
- `REX` prefixes have 16 possible lower-4-bits. Test more possibilities (I typically only testedthe REX bytes `0x40` and `0x41`, but they go up to `0x4F`).

## Tooling relevant for development

`objdump` is great for giving the segments etc. of an ELF file with the `-x` flag, but it doesn't seem to disassemble (`-d`) the ELFs produced by DefAssembler. `readelf` works about as well as `objdump`.

For quick things, https://defuse.ca/online-x86-assembler.htm is good for assembly/disassembling small bits of asm. It's not perfectly reliable though, and make sure to switch to x64 architecture.

`gdb` can show disassembly with the `disassemble` command. For example, `disassemble /m 0x400000, +0x49` disassembles the 73 bytes starting at `0x400000`.

`nm` is irrelevant because we tend to not have any symbols to deal with.
