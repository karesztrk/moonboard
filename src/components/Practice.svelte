<script lang="ts">
  import { onMount } from "svelte";
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
    console.log(e.key, e.shiftKey);
    if (e.key === letters[caret]) {
      caret++;
    }
  };

  const restart = () => {
    caret = 0;
  };

  const focus = () => {
    setTimeout(() => {
      ref.focus();
      console.log(document.activeElement);
    }, 0);
  };

  let ref: HTMLInputElement;

  onMount(() => {
    focus();
  });
</script>

<input
  bind:this={ref}
  type="text"
  autocomplete="off"
  autocapitalize="none"
  autocorrect="off"
  spellcheck="false"
  on:keydown|preventDefault={onKeyDown}
  on:blur={focus}
/>
<blockquote>
  <p>
    <caret />
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

<style>
  blockquote {
    --caret-color: #000;
  }

  blockquote > p {
    position: relative;
  }

  caret {
    position: absolute;
    display: block;
    height: 1em;
    width: 0.1em;
    top: 0;
    left: 0;
    font-size: 1.5rem;
    animation: linear 1s infinite alternate blink;
    background-color: var(--caret-color);
  }

  input {
    display: inline-block;
    position: absolute;
    overflow: hidden;
    clip: rect(0 0 0 0);
    height: 1;
    width: 1;
    margin: -1;
    padding: 0;
    border: 0;
  }

  @keyframes blink {
    0%,
    50% {
      opacity: 1;
    }

    51%,
    100% {
      opacity: 0;
    }
  }
</style>
