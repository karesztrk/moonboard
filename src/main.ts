import { invoke } from "@tauri-apps/api/core";

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  // eslint-disable-next-line
  const target = e.target as HTMLFormElement;
  const data = new FormData(target, e.submitter);
  const key = (data.get("key") as string | null) || "";

  if (!key || key.length === 0) {
    return;
  }

  invoke("light_on_key", { key: key.charAt(0) })
    .then((result) => {
      // eslint-disable-next-line
      console.log(result);
    })
    .catch((error) => {
      console.error(error);
    });
}

window.addEventListener("DOMContentLoaded", () => {
  const form = document.querySelector("#greet-form") as HTMLFormElement | null;
  if (form) {
    form.addEventListener("submit", handleSubmit);
  }
});
