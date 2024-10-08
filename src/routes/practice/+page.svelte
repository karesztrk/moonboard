<script lang="ts">
  import Quote from "$lib/components/Quote.svelte";
  import type { Layout } from "$lib/types";
  import practiceMachine from "$lib/stores/practice";
  import services from "$lib/stores/services";
  import { onMount } from "svelte";

  $: ({ state, context } = $practiceMachine);

  $: layout = "Norman" as Layout;

  const layouts = ["Norman", "Qwerty"];

  const onLetterChange = (e: CustomEvent<{ letter: string }>) => {
    $services.keyboard
      .clear()
      .then(() =>
        $services.keyboard.lightOnKey(e.detail.letter.toLowerCase(), layout),
      );
  };

  const onSubmit = (e: SubmitEvent) => {
    const target = e.target as HTMLFormElement;
    const data = new FormData(target);

    layout = data.get("layout") as Layout;
    practiceMachine.send("START");
  };

  const onPause = (e: Event) => {
    e.stopPropagation();
    practiceMachine.send("PAUSE");
  };

  const onRestart = (e: Event) => {
    e.stopPropagation();
    practiceMachine.send("RESUME");
  };

  const onStop = (e: Event) => {
    e.stopPropagation();
    practiceMachine.send("STOP");
  };

  onMount(() => {
    return () => {
      practiceMachine.send("STOP");
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
      author={context.author}
      text={context.text}
      on:pause={onPause}
      on:restart={onRestart}
      on:stop={onStop}
      on:letterchange={onLetterChange}
    />
  {/if}
</section>
