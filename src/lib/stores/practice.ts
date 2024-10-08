import machine from "./machine";
import quotes from "$lib/assets/quote.json";

export type State = "idle" | "running" | "paused" | "finished";

export type Event = "START" | "PAUSE" | "RESUME" | "STOP" | "FINISH";

const pickRandomQuote = () => {
  return quotes[Math.floor(Math.random() * quotes.length)];
};

const practiceMachine = machine<State, Event, { text: string; author: string }>(
  "idle",
  {
    text: "",
    author: "",
  },
  {
    idle: {
      on: {
        START: {
          target: "running",
          action: () => {
            const { text, author } = pickRandomQuote();
            return {
              text,
              author,
            };
          },
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

export default practiceMachine;
