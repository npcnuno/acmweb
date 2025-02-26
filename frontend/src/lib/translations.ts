// src/lib/i18n/index.ts
import { browser } from "$app/environment";
import { init, register } from "svelte-i18n";

const defaultLocale = "en";

register("en", () => import("./locales/en.json"));
register("de", () => import("./locales/de.json"));

register("pt", () => import("./locales/pt.json"));
register("ja", () => import("./locales/ja.json"));
register("fr", () => import("./locales/fr.json"));
register("zh", () => import("./locales/zh.json"));
init({
  fallbackLocale: defaultLocale,
  initialLocale: browser ? window.navigator.language : defaultLocale,
});
