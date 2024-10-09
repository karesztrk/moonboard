<script lang="ts">
  import HiddenInput from "$lib/components/HiddenInput.svelte";
  import { onMount } from "svelte";

  export let onKeyDown: (e: KeyboardEvent) => void;
  export let onStop: () => void;

  let inputElement: HTMLInputElement;

  const focusInput = () => {
    inputElement.focus();
  };

  onMount(() => {
    setTimeout(() => {
      focusInput();
    }, 0);
    return () => {
      onStop();
    };
  });
</script>

<p>Press any letter</p>

<HiddenInput bind:input={inputElement} {onKeyDown} />
<div>
  <button on:click={onStop}>Stop</button>
</div>
