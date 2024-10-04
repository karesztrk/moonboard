<script lang="ts">
  import Practice from "@/components/Practice.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type { Layout } from "@/types";
  import { practiceMachine } from "@/store/store";
  import { onMount } from "svelte";

  $: ({ state } = $practiceMachine);
  $: layout = "Norman" as Layout;

  const layouts = ["Norman", "Qwerty"];

  async function resetLight() {
    await invoke("reset", {});
  }

  async function lightOnKey(key: string) {
    await invoke("light_on_key", { key, layout });
  }

  async function clear() {
    await invoke("clear", {});
  }

  const onLetterChange = (e: CustomEvent<{ letter: string }>) => {
    clear().then(() => lightOnKey(e.detail.letter.toLowerCase()));
  };

  const onSubmit = (e: SubmitEvent) => {
    const target = e.target as HTMLFormElement;
    const data = new FormData(target);

    layout = data.get("layout") as Layout;
    practiceMachine.send("START");
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
    <Practice on:letterchange={onLetterChange} />
  {/if}
</section>
