import machine from "./machine";

export type State = "idle" | "running" | "paused" | "finished";

export type Event = "START" | "PAUSE" | "RESUME" | "STOP" | "FINISH";

const gameMachine = machine<State, Event, {}>(
  "idle",
  {},
  {
    idle: {
      on: {
        START: {
          target: "running",
        },
      },
    },
    running: {
      on: {
        PAUSE: { target: "paused" },
        STOP: { target: "idle" },
        FINISH: { target: "finished" },
      },
    },
    paused: {
      on: {
        RESUME: { target: "running" },
        STOP: { target: "idle" },
      },
    },
    finished: {
      on: {
        STOP: { target: "idle" },
      },
    },
  },
);

export default gameMachine;
