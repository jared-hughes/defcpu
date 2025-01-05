export interface RunCode {
  type: "run";
  src: string;
}

export type MessageToWorker = RunCode;

export interface GotResult {
  type: "result";
  stdout: string;
  stderr: string;
}

export type MessageFromWorker = GotResult;
