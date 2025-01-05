/* tslint:disable */
/* eslint-disable */
export class OuterMachine {
  private constructor();
  free(): void;
  static init(input: Uint8Array): OuterMachine;
  step(): void;
  is_done(): boolean;
  get_stdout(): Uint8Array;
  get_stderr(): Uint8Array;
  get_registers_str(): Uint8Array;
  get_rip(): bigint;
  get_full_step_count(): bigint;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_outermachine_free: (a: number, b: number) => void;
  readonly outermachine_init: (a: number, b: number) => number;
  readonly outermachine_step: (a: number) => void;
  readonly outermachine_is_done: (a: number) => number;
  readonly outermachine_get_stdout: (a: number, b: number) => void;
  readonly outermachine_get_stderr: (a: number, b: number) => void;
  readonly outermachine_get_registers_str: (a: number, b: number) => void;
  readonly outermachine_get_rip: (a: number) => bigint;
  readonly outermachine_get_full_step_count: (a: number) => bigint;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
