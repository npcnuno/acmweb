import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import wasmPack from "vite-plugin-wasm-pack";
import { purgeCss } from "vite-plugin-tailwind-purgecss";

export default defineConfig({
  plugins: [sveltekit(), wasmPack("./wasm-test"), purgeCss()],
});
