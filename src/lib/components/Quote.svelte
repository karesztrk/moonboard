<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { onMount } from "svelte";
  import Caret from "./Caret.svelte";
  import type { State } from "$lib/stores/practice";

  export let text = "";
  export let author = "";
  export let state: State = "idle";

  $: letters = text.toLowerCase().split("");

  $: wordIndex = 0;
  $: initialCaretPosition = { left: 0, top: 0 };
  $: caretPosition = { left: 0, top: 0 };

  const dispatch = createEventDispatcher();

  const emitChangeEvent = () => {
    dispatch("letterchange", { letter: letters[wordIndex] });
  };

  const updateCaretPosition = (newPosition = { left: 0, top: 0 }) => {
    caretPosition = {
      left: newPosition.left - initialCaretPosition.left,
      top: newPosition.top - initialCaretPosition.top,
    };
  };

  const onKeyDown = (e: KeyboardEvent) => {
    if (e.key === letters[wordIndex]) {
      wordIndex++;
      emitChangeEvent();
      moveCaret(getCurrentLetter);
      if (wordIndex === letters.length) {
        dispatch("finish");
      }
    } else if (e.key === "Backspace" && wordIndex === 1) {
      onRestart();
      emitChangeEvent();
    } else if (e.key === "Backspace" && wordIndex > 0) {
      wordIndex--;
      emitChangeEvent();
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
      const rect = current.getBoundingClientRect();
      updateCaretPosition({
        left: rect.right,
        top: rect.top,
      });
    }
  };

  const measureInitialCaretPosition = () => {
    const current = getCurrentLetter();
    if (current) {
      const rect = current.getBoundingClientRect();
      initialCaretPosition = {
        left: rect.left,
        top: rect.top,
      };
    }
  };

  const onBack = () => {
    wordIndex = 0;
    updateCaretPosition(initialCaretPosition);
    dispatch("stop");
  };

  const onPause = () => {
    dispatch("pause");
  };

  const onRestart = () => {
    dispatch("restart");
  };

  const focusInput = () => {
    inputElement.focus();
  };

  let inputElement: HTMLInputElement;

  onMount(() => {
    emitChangeEvent();
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
    on:blur={onPause}
    on:focus={onRestart}
  />
  <p>
    {#if state === "running"}
      <Caret
        style={`translate: ${caretPosition.left}px ${caretPosition.top}px;`}
      />
    {/if}
    {#each letters as letter, i}
      <letter
        class:correct={i < wordIndex}
        class:incorrect={i >= wordIndex}
        data-previous-guess={i === wordIndex - 2 ? "" : undefined}
        data-current={i === wordIndex ? "" : undefined}>{letter}</letter
      >
    {/each}
    {#if state === "finished"}
      <span>🎉</span>
    {/if}
  </p>
  <footer>
    <cite>— {author}</cite>
  </footer>
</blockquote>
{#if state === "finished"}
  <button on:click={onBack}>Back</button>
{/if}

<style>
  blockquote {
    --_font-size: 3rem;
    filter: blur(4px);
  }

  blockquote > p {
    font-size: var(--_font-size);
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
</style>
