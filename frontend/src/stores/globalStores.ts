// src/stores/globalStores.ts
import { writable } from "svelte/store";

export const authToken = writable<string | null>(null);
export const refreshToken = writable<string | null>(null);
export const userData = writable<any>(null);
export const postsStore = writable<any[]>([]);
export const projectsStore = writable<any[]>([]);
