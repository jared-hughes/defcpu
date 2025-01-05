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
}

export interface MsgStatus {
  type: "status";
  status: Status;
}

export interface MsgDone {
  type: "done";
  status: Status;
}

export type MessageFromWorker = MsgStatus | MsgDone;
