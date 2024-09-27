<script lang="ts">
  import { onMount } from "svelte";
  // import { invoke } from "@tauri-apps/api/core";
  import quotes from "@/assets/quote.json";

  let { text: txt, author } = quotes[Math.floor(Math.random() * quotes.length)];
  let letters = txt.split("");
  $: caret = 0;
  $: initialCaretPosition = 0;
  $: finish = caret === letters.length;
  $: running = false;

  // async function greet() {
  //   greetMsg = await invoke("greet", { name });
  // }
  //
  const updateCaretPosition = (newPosition = 0) => {
    const delta = newPosition - initialCaretPosition;
    carretElement.style.translate = `${delta}px`;
  };

  const onKeyDown = (e: KeyboardEvent) => {
    if (e.key === letters[caret]) {
      caret++;
      measureCaretPosition();
    }
  };

  const getCurrentLetter = () => {
    return document.querySelector('[data-current="true"]');
  };

  const measureCaretPosition = () => {
    const current = getCurrentLetter();
    if (current) {
      const newPosition = current.getBoundingClientRect().right;
      updateCaretPosition(newPosition);
    }
  };

  const measureInitialCaretPosition = () => {
    const current = getCurrentLetter();
    if (current) {
      initialCaretPosition = current.getBoundingClientRect().left;
    }
  };

  const restart = () => {
    caret = 0;
    updateCaretPosition(initialCaretPosition);
  };

  const stop = () => {
    running = false;
  };

  const start = () => {
    running = true;
  };

  const focusInput = () => {
    inputElement.focus();
  };

  let inputElement: HTMLInputElement;
  let carretElement: HTMLDivElement;

  onMount(() => {
    setTimeout(() => {
      focusInput();
      measureInitialCaretPosition();
    }, 0);
  });
</script>

<blockquote on:click={focusInput}>
  <input
    bind:this={inputElement}
    type="text"
    autocomplete="off"
    autocapitalize="none"
    autocorrect="off"
    spellcheck="false"
    on:keydown|preventDefault={onKeyDown}
    on:blur={stop}
    on:focus={start}
  />
  <p>
    <caret class:running={running && !finish} bind:this={carretElement} />
    {#each letters as letter, i}
      <letter
        class:correct={i < caret}
        class:incorrect={i >= caret}
        data-current={i === caret ? "true" : undefined}>{letter}</letter
      >
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
    filter: blur(4px);
  }

  blockquote > p {
    position: relative;
  }

  blockquote:focus-within {
    filter: none;
  }

  caret {
    position: absolute;
    display: none;
    height: 1em;
    width: 0.1em;
    top: 0;
    left: 0;
    font-size: 1.5rem;
    background-color: var(--caret-color);
    transition: translate 200ms ease-out;
  }

  caret.running {
    display: block;
    animation: linear 1s infinite alternate blink;
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

  .correct {
    opacity: 1;
  }

  .incorrect {
    opacity: 0.25;
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
