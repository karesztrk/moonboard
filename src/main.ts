import { invoke } from "@tauri-apps/api/core";
import { Text } from "./components/Text";

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  const target = e.target as HTMLFormElement;
  const data = new FormData(target, e.submitter);
  const keyData = (data.get("key") as string | null) || "";

  if (!keyData || keyData.length === 0) {
    return;
  }

  const key = keyData.charAt(0);

  invoke("light_on_key", { key })
    .then((result) => {
      console.log(result);
    })
    .catch((error) => {
      console.error(error);
    });
  target.reset();

  window.addEventListener("keydown", (e: KeyboardEvent) => {
    if (e.key === key) {
      console.log("key down", e.key);

      invoke("clear")
        .then((result) => {
          console.log(result);
        })
        .catch((error) => {
          console.error(error);
        });
    }
  });
}

window.addEventListener("DOMContentLoaded", () => {
  const form = document.querySelector("#greet-form") as HTMLFormElement | null;
  if (form) {
    form.addEventListener("submit", handleSubmit);
  }
});
