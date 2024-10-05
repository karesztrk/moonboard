<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { onMount } from "svelte";
  import Caret from "./Caret.svelte";
  import practiceMachine from "$lib/stores/practice";

  $: ({ state, context } = $practiceMachine);
  $: letters = context.quote.toLowerCase().split("");

  $: wordIndex = 0;
  $: initialCaretPosition = 0;
  $: caretPosition = 0;
  $: finish = wordIndex === letters.length;

  const dispatch = createEventDispatcher();

  const emitChangeEvent = () => {
    dispatch("letterchange", { letter: letters[wordIndex] });
  };

  const updateCaretPosition = (newPosition = 0) => {
    caretPosition = newPosition - initialCaretPosition;
  };

  const onKeyDown = (e: KeyboardEvent) => {
    if (e.key === letters[wordIndex]) {
      wordIndex++;
      emitChangeEvent();
      moveCaret(getCurrentLetter);
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

  const onBack = () => {
    wordIndex = 0;
    updateCaretPosition(initialCaretPosition);
    practiceMachine.send("STOP");
  };

  const onPause = () => {
    practiceMachine.send("PAUSE");
  };

  const onRestart = () => {
    practiceMachine.send("RESUME");
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
    <Caret
      running={state === "running" && !finish}
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
{#if finish}
  <button on:click={onBack}>Back</button>
{/if}

<style>
  blockquote {
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
</style>
