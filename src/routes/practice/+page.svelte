<script lang="ts">
  import Practice from "@/components/Practice.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import quotes from "@/assets/quote.json";
  import type { State, Layout } from "@/types";

  $: state = "READY" as State;
  $: text = pickRandomQuote().text;
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

  const pickRandomQuote = () => {
    return quotes[Math.floor(Math.random() * quotes.length)];
  };

  const onSubmit = (e: SubmitEvent) => {
    const target = e.target as HTMLFormElement;
    const data = new FormData(target);

    layout = data.get("layout") as Layout;
    console.log(layout);
    state = "RUNNING";
  };

  const onStart = () => {
    state = "RUNNING";
  };

  const onPause = () => {
    state = "PAUSED";
  };

  const onRestart = () => {
    state = "READY";
    text = pickRandomQuote().text;

    resetLight();
  };
</script>

<section>
  <h2>Practice</h2>
  <p>Practice typing by hitting the right keyboard buttons</p>

  {#if state !== "RUNNING"}
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
    <Practice
      on:letterchange={onLetterChange}
      on:start={onStart}
      on:pause={onPause}
      on:restart={onRestart}
      {state}
      {text}
    />
  {/if}
</section>
