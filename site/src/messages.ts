export interface MsgRunCode {
  type: "run";
  src: string;
}

export type MessageToWorker =
  | MsgRunCode
  | { type: "poll-status" | "halt" | "pause" | "continue" | "single-step" };

export interface Status {
  stdout: string;
  stderr: string;
  registersStr: string;
  rip: bigint;
  fullStepCount: bigint;
}

export interface MsgStatus {
  type: "status";
  status: Status;
}

export interface MsgDone {
  type: "done";
  status: Status;
}

export interface MsgError {
  type: "error";
  error: string;
}

export type MessageFromWorker = MsgStatus | MsgDone | MsgError;
