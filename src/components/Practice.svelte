<script lang="ts">
  import { onMount } from "svelte";
  // import { invoke } from "@tauri-apps/api/core";
  import quotes from "@/assets/quote.json";
  import Caret from "./Caret.svelte";

  let { text: txt, author } = quotes[Math.floor(Math.random() * quotes.length)];
  let letters = txt.split("");
  $: wordIndex = 0;
  $: initialCaretPosition = 0;
  $: caretPosition = 0;
  $: finish = wordIndex === letters.length;
  $: running = false;

  // async function greet() {
  //   greetMsg = await invoke("greet", { name });
  // }
  //
  const updateCaretPosition = (newPosition = 0) => {
    caretPosition = newPosition - initialCaretPosition;
  };

  const onKeyDown = (e: KeyboardEvent) => {
    if (e.key === letters[wordIndex]) {
      wordIndex++;
      moveCaret(getCurrentLetter);
    } else if (e.key === "Backspace" && wordIndex === 1) {
      restart();
    } else if (e.key === "Backspace" && wordIndex > 0) {
      wordIndex--;
      moveCaret(getLastGuess);
    }
  };

  const getCurrentLetter = () => {
    return document.querySelector("[data-current]");
  };

  const getLastGuess = () => {
    return document.querySelector("[data-previous-guess]");
  };

  const moveCaret = (letterFn = getCurrentLetter) => {
    const current = letterFn();
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
    wordIndex = 0;
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
    <Caret
      running={running && !finish}
      style={`translate: ${caretPosition}px`}
    />
    {#each letters as letter, i}
      <letter
        class:correct={i < wordIndex}
        class:incorrect={i >= wordIndex}
        data-previous-guess={i === wordIndex - 2 ? "" : undefined}
        data-current={i === wordIndex ? "" : undefined}>{letter}</letter
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
