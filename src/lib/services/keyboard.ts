import { invoke } from "@tauri-apps/api/core";

class KeyboardService {
  async resetLight() {
    await invoke("reset", {});
  }

  async lightOnKey(key: string, layout: string) {
    await invoke("light_on_key", { key, layout });
  }

  async clear() {
    await invoke("clear", {});
  }

  async wipeAnimation() {
    await invoke("wipe_animation", {});
  }

  async sequenceAnimation() {
    await invoke("sequence_animation", {});
  }

  async torpedoOnKey(key: string, layout: string) {
    await invoke("torpedo_on_key", { key, layout });
  }
}

export default KeyboardService;
