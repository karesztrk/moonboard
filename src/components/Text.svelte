<script lang="ts">
  // import { invoke } from "@tauri-apps/api/core";
  import quotes from "@/assets/quote.json";

  let { text: txt, author } = quotes[Math.floor(Math.random() * quotes.length)];
  let letters = txt.split("");
  $: caret = 0;
  $: finish = caret === letters.length;

  // async function greet() {
  //   greetMsg = await invoke("greet", { name });
  // }
  //
  const onKeyDown = (e: KeyboardEvent) => {
    if (e.key === letters[caret]) {
      caret++;
    }
  };

  const restart = () => {
    caret = 0;
  };
</script>

<div>
  <blockquote>
    <p>
      {#each letters as letter, i}
        <letter style="opacity: {caret > i ? 1 : 0.25}">{letter}</letter>
      {/each}
      {#if finish}
        <span>🎉</span>
      {/if}
    </p>
  </blockquote>
  <p>
    —{author}
  </p>
  {#if finish}
    <button on:click={restart}>Restart</button>
  {/if}
</div>

<svelte:window on:keydown|preventDefault={onKeyDown} />
