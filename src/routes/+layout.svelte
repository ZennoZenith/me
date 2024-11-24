<script lang="ts">
export const prerendered = true;

import "../app.css";
import Footer from "./footer.svelte";
import Header from "./header.svelte";

let { children, data } = $props();
</script>

<div class="layout m-auto p-8 2xl:p-0">
  <Header />

  <main class="p-16">
    {@render children?.()}
  </main>

  <Footer />
</div>

<style>
.layout {
  height: 100%;
  max-inline-size: 1440px;
  display: grid;
  grid-template-rows: auto 1fr auto;
}
</style>
