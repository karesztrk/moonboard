<script lang="ts">
  let carretElement: HTMLDivElement;
  export let running = false;
  export let style: string = "";
</script>

<caret class:running bind:this={carretElement} {style} />

<style>
  caret {
    --_caret-color: var(--pico-contrast);
    position: absolute;
    display: none;
    height: 1em;
    width: 0.1em;
    top: 0;
    left: 0;
    font-size: 1.5rem;
    background-color: var(--_caret-color);
    transition: translate 200ms ease-out;
  }

  caret.running {
    display: block;
    animation: linear 1s infinite alternate blink;
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
