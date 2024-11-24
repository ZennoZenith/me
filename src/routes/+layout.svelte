<script lang="ts">
import "../app.css";
export const prerendered = true;

let { children } = $props();
</script>

{@render children()}
