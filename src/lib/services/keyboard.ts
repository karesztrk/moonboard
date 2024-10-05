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
}

export default KeyboardService;
