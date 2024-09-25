<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Text from "@/components/Text.svelte";

  let name = "";
  let greetMsg = "";

  async function greet() {
    greetMsg = await invoke("greet", { name });
  }
</script>

<section>
  <h2>Moontype</h2>
  <p>Practice typing by hitting the right keyboard buttons</p>

  <Text />
</section>
