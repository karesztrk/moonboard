<script lang="ts">
  import Practice from "@/components/Practice.svelte";
  import { invoke } from "@tauri-apps/api/core";

  const layoutTypes = {
    Qwerty: "Qwerty",
    Norman: "Norman",
  } as const;

  async function lightOnKey(key: string) {
    await invoke("light_on_key", { key, layout: layoutTypes.Norman });
  }

  async function clear() {
    await invoke("clear", {});
  }

  const onLetterChange = (e: CustomEvent<{ letter: string }>) => {
    clear().then(() => lightOnKey(e.detail.letter.toLowerCase()));
  };
</script>

<section>
  <h2>Moontype</h2>
  <p>Practice typing by hitting the right keyboard buttons</p>

  <Practice on:letterchange={onLetterChange} />
</section>
