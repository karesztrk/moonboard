<script lang="ts">
  import practiceMachine from "$lib/stores/practice";
  import services from "$lib/stores/services";
  import { onMount } from "svelte";

  const layouts = ["Norman", "Qwerty"];

  type Layout = (typeof layouts)[number];

  let layout = "Norman" as Layout;

  $: ({ state, context } = $practiceMachine);

  const onSubmit = (e: SubmitEvent) => {
    const target = e.target as HTMLFormElement;
    const data = new FormData(target);

    layout = data.get("layout") as Layout;
    $services.keyboard.clear();
    practiceMachine.send("START");
  };

  const onKeyDown = (e: KeyboardEvent) => {
    $services.keyboard.torpedoOnKey(e.key.toLowerCase(), layout);
  };

  const onStop = () => {
    practiceMachine.send("FINISH");
  };

  onMount(() => {
    return () => {
      practiceMachine.send("STOP");
    };
  });
</script>

<section>
  <h2>Practice</h2>
  <p>Play torpedo on your keyboard</p>

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

      <button type="submit">Play</button>
    </form>
  {:else}
    Press any letter

    <input
      type="text"
      autocomplete="off"
      autocapitalize="none"
      autocorrect="off"
      spellcheck="false"
      on:keydown|preventDefault={onKeyDown}
    />
    <button
      on:click={() => {
        practiceMachine.send("STOP");
      }}>Stop</button
    >
  {/if}
</section>
