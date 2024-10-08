<script lang="ts">
  import services from "$lib/stores/services";

  const animations = ["Wipe"];
  const onSubmit = (e: SubmitEvent) => {
    $services.keyboard.clear().then(() => $services.keyboard.animate());
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
