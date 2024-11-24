import { mdsvex } from "mdsvex";
// import adapter from '@sveltejs/adapter-auto';
// import adapter from '@sveltejs/adapter-node';
import staticAdapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: [vitePreprocess(), mdsvex()],
  kit: {
    adapter: staticAdapter({
      pages: "build",
      assets: "build",
      fallback: undefined,
      precompress: true,
      strict: true,
    }),
  },
  extensions: [".svelte", ".svx"],
  compilerOptions: {
    runes: true,
  },
};

export default config;
