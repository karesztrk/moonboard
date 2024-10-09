<script lang="ts">
  export let onKeyDown: (e: KeyboardEvent) => void;
  export let onPause: () => void | undefined;
  export let onResume: () => void | undefined;

  export let input: HTMLInputElement;
</script>

<input
  class="hidden"
  bind:this={input}
  type="text"
  autocomplete="off"
  autocapitalize="none"
  autocorrect="off"
  spellcheck="false"
  on:keydown|preventDefault={onKeyDown}
  on:blur={onPause}
  on:focus={onResume}
/>

<style>
  input.hidden {
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
</style>
