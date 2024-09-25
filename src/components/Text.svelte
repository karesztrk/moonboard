<script lang="ts">
  // import { invoke } from "@tauri-apps/api/core";

  let txt = "lorem ipsum";
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
  {#each letters as letter, i}
    <letter style="opacity: {caret > i ? 1 : 0.25}">{letter}</letter>
  {/each}
  <p>{finish ? "🎉" : ""}</p>
  {#if finish}
    <button on:click={restart}>Restart</button>
  {/if}
</div>

<svelte:window on:keydown|preventDefault={onKeyDown} />
