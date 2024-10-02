import { writable } from "svelte/store";

type State = string;

type Event = string;

type TransitionFunction<S extends State> = (state: S) => S;

// Generic transitions object type
type Transitions<S extends State, E extends Event> = {
  [K in S]: {
    [J in E]?: TransitionFunction<S>;
  };
};

// Generic state machine interface
interface StateMachine<S extends State, E extends Event> {
  subscribe: (callback: (state: S) => void) => () => void;
  send: (event: E) => void;
}

const createStateMachine = <S extends State, E extends Event>(
  initialState: S,
  transitions: Transitions<S, E>,
): StateMachine<S, E> => {
  const { subscribe, update } = writable<S>(initialState);

  const send = (event: E) => {
    update((state) => {
      const transition = transitions[state][event];
      return transition ? transition(state) : state;
    });
  };

  return { subscribe, send };
};

export default createStateMachine;
