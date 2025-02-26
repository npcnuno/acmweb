// src/stores/alertStore.ts
import { writable } from "svelte/store";

// This store holds the current alert message, or null if no alert is active.
export const alertMessage = writable<string | null>(null);

/**
 * Displays an alert message for a given duration.
 * @param message - The message to display.
 * @param duration - Duration in milliseconds for the alert to be visible (default: 3000).
 */
export function showAlert(message: string, duration: number = 3000): void {
  alertMessage.set(message);
  setTimeout(() => {
    alertMessage.set(null);
  }, duration);
}
