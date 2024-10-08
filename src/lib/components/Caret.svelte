<script lang="ts">
  let carretElement: HTMLDivElement;
  export let style: string = "";
</script>

<caret bind:this={carretElement} {style} />

<style>
  caret {
    --_caret-color: var(--pico-contrast);
    position: absolute;
    height: 1em;
    width: 3px;
    top: 0.3em;
    left: 0;
    background-color: var(--_caret-color);
    transition: translate 200ms ease-out;
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
