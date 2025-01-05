export interface MsgRunCode {
  type: "run";
  src: string;
}

export interface MsgPollStatus {
  type: "poll-status";
}

export type MessageToWorker = MsgRunCode | MsgPollStatus;

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
