import machine from "./machine";

export type State = "idle" | "running" | "paused" | "finished";
type TimerEvent = "START" | "PAUSE" | "RESUME" | "STOP";

export const practiceMachine = machine<State, TimerEvent>("idle", {
  idle: {
    START: () => "running",
  },
  running: {
    PAUSE: () => "paused",
    STOP: () => "idle",
  },
  paused: {
    RESUME: () => "running",
    STOP: () => "idle",
  },
  finished: {
    STOP: () => "idle",
  },
});
