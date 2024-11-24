// import adapter from "@sveltejs/adapter-auto";
import adapter from "@sveltejs/adapter-node";
// import staticAdapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { mdsvex } from "mdsvex";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: [vitePreprocess(), mdsvex()],
  kit: {
    adapter: adapter(),
  },
  extensions: [".svelte", ".svx"],
  compilerOptions: {
    runes: true,
  },
};

export default config;
