import { writable } from "svelte/store";

export const selected_button_url = writable(
  // Current url in use
  "",
);
