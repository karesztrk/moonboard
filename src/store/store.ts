import machine from "./machine";
import quotes from "@/assets/quote.json";

export type State = "idle" | "running" | "paused" | "finished";

type Event = "START" | "PAUSE" | "RESUME" | "STOP";

const pickRandomQuote = () => {
  return quotes[Math.floor(Math.random() * quotes.length)];
};

export const practiceMachine = machine<State, Event, { quote: string }>(
  "idle",
  {
    quote: "",
  },
  {
    idle: {
      on: {
        START: {
          target: "running",
          action: () => ({
            quote: pickRandomQuote().text,
          }),
        },
      },
    },
    running: {
      on: {
        PAUSE: { target: "paused" },
        STOP: { target: "idle" },
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
