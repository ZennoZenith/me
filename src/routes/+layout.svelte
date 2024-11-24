<script lang="ts">
import "../app.css";
export const prerendered = true;
export const trailingSlash = "always";

let { children } = $props();
</script>

{@render children()}
