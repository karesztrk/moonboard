<script lang="ts">
  import { onMount } from "svelte";
  import Caret from "./Caret.svelte";
  import type { State } from "$lib/stores/practice";
  import HiddenInput from "./HiddenInput.svelte";

  export let text = "";
  export let author = "";
  export let state: State = "idle";
  export let onPause: () => void;
  export let onResume: () => void;
  export let onStop: () => void;
  export let onFinish: () => void;
  export let onLetterChange: (letter: string) => void;
  let inputElement: HTMLInputElement;

  $: letters = text.toLowerCase().split("");

  $: wordIndex = 0;
  $: initialCaretPosition = { left: 0, top: 0 };
  $: caretPosition = { left: 0, top: 0 };

  const emitChangeEvent = () => {
    onLetterChange(letters[wordIndex]);
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
        onFinish();
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

  const onRestart = () => {
    wordIndex = 0;
    updateCaretPosition(initialCaretPosition);
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
    onStop();
  };

  const focusInput = () => {
    inputElement.focus();
  };

  onMount(() => {
    emitChangeEvent();
    setTimeout(() => {
      focusInput();
      measureInitialCaretPosition();
    }, 0);
  });
</script>

<blockquote on:click={focusInput}>
  <HiddenInput bind:input={inputElement} {onKeyDown} {onPause} {onResume} />
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

  .correct {
    opacity: 1;
  }

  .incorrect {
    opacity: 0.25;
  }
</style>
