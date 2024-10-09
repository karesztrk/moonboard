<script lang="ts">
  import gameMachine from "$lib/stores/game";
  import services from "$lib/stores/services";
  import { onMount } from "svelte";
  import { type Layout, layouts } from "$lib/types";
  import Game from "$lib/components/Game.svelte";

  let layout = "Norman" as Layout;

  $: ({ state } = $gameMachine);

  const onSubmit = (e: SubmitEvent) => {
    const target = e.target as HTMLFormElement;
    const data = new FormData(target);

    layout = data.get("layout") as Layout;
    $services.keyboard.clear();
    gameMachine.send("START");
  };

  const onKeyDown = (e: KeyboardEvent) => {
    $services.keyboard.torpedoOnKey(e.key.toLowerCase(), layout);
  };

  const onStop = () => {
    gameMachine.send("STOP");
  };

  onMount(() => {
    return () => {
      gameMachine.send("STOP");
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
    <Game {onKeyDown} {onStop} />
  {/if}
</section>
