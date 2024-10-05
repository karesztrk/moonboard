import { readable } from "svelte/store";
import KeyboardService from "$lib/services/keyboard";

const services = readable({
  keyboard: new KeyboardService(),
});

export default services;
