<script lang="ts">
  import services from "$lib/stores/services";

  const animations = ["Sequence", "Wipe"] as const;
  const onSubmit = (e: SubmitEvent) => {
    const target = e.target as HTMLFormElement;
    const data = new FormData(target);

    const name = data.get("name") as (typeof animations)[number];
    switch (name) {
      case "Wipe":
        $services.keyboard.wipeAnimation();
        break;
      case "Sequence":
        $services.keyboard.sequenceAnimation();
      default:
        break;
    }
  };
</script>

<section>
  <h2>Practice</h2>
  <p>Play an animation with your keyboard</p>

  <form on:submit|preventDefault={onSubmit}>
    <fieldset>
      <label for="name">Name</label>
      <select id="name" name="name" required>
        {#each animations as animation}
          <option value={animation}>{animation}</option>
        {/each}
      </select>
      <small>Pick an animation</small>
    </fieldset>
    <button type="submit">Play</button>
  </form>
</section>
