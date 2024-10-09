<script lang="ts">
  import Quote from "$lib/components/Quote.svelte";
  import gameMachine, { type Event } from "$lib/stores/game";
  import services from "$lib/stores/services";
  import { onMount } from "svelte";
  import Confetti from "$lib/components/Confetti.svelte";
  import { type Layout, layouts } from "$lib/types";
  import quotes from "$lib/assets/quote.json";

  $: ({ state } = $gameMachine);

  $: layout = "Norman" as Layout;

  $: quote = { text: "", author: "" };

  const onLetterChange = (letter: string) => {
    $services.keyboard
      .clear()
      .then(() => $services.keyboard.lightOnKey(letter.toLowerCase(), layout));
  };

  const pickRandomQuote = () => {
    return quotes[Math.floor(Math.random() * quotes.length)];
  };

  const onSubmit = (e: SubmitEvent) => {
    const target = e.target as HTMLFormElement;
    const data = new FormData(target);

    layout = data.get("layout") as Layout;
    quote = pickRandomQuote();
    gameMachine.send("START");
  };

  const onEvent = (machineEvent: Event) => () => {
    gameMachine.send(machineEvent);
  };

  onMount(() => {
    return () => {
      gameMachine.send("STOP");
    };
  });
</script>

<section>
  <h2>Practice</h2>
  <p>Practice typing by hitting the right keyboard buttons</p>
  {#if state === "idle"}
    <form on:submit|preventDefault={onSubmit}>
      <fieldset>
        <label for="layout">Layout</label>
        <select id="layout" name="layout" required>
          {#each layouts as layout}
            <option value={layout}>{layout}</option>
          {/each}
        </select>
        <small>Set how your keyboard is laid out</small>
      </fieldset>

      <div>
        <button type="submit">Start</button>
      </div>
    </form>
  {:else}
    <Quote
      {state}
      author={quote.author}
      text={quote.text}
      onPause={onEvent("PAUSE")}
      onResume={onEvent("RESUME")}
      onStop={onEvent("STOP")}
      onFinish={onEvent("FINISH")}
      {onLetterChange}
    />
  {/if}
  {#if state === "finished"}
    <Confetti />
  {/if}
</section>
