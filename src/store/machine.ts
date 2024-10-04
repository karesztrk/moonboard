import { writable } from "svelte/store";

type State = string;

type Event = string;

type Config<S extends State, E extends Event, C> = {
  [K in S]: {
    on?: {
      [J in E]?: {
        target: S;
        action?: (context: C) => Partial<C>;
      };
    };
  };
};

type MachineState<TState extends string, TContext> = {
  state: TState;
  context: TContext;
};

interface StateMachine<S extends State, E extends Event, C> {
  subscribe: (callback: (state: MachineState<S, C>) => void) => () => void;
  send: (event: E) => void;
}

const createStateMachine = <
  S extends State,
  E extends Event,
  C extends Record<string, string | number | boolean>,
>(
  initialState: S,
  initialContext: C,
  config: Config<S, E, C>,
): StateMachine<S, E, C> => {
  const { subscribe, update } = writable<MachineState<S, C>>({
    state: initialState,
    context: initialContext,
  });

  const send = (event: E) => {
    update((current) => {
      const stateConfig = config[current.state];
      const transition = stateConfig.on?.[event];

      if (!transition) return current;

      const newContext = transition.action
        ? { ...current.context, ...transition.action(current.context) }
        : current.context;

      return {
        state: transition.target,
        context: newContext,
      };
    });
  };

  return { subscribe, send };
};

export default createStateMachine;
