<script lang="ts">
import * as config from "$lib/config";
</script>

<footer class="p-8">
  <p class="">{config.title} &copy {new Date().getFullYear()}</p>
</footer>

<style>
footer {
  padding-block: var(--size-7);
  border-top: 1px solid var(--border);

  p {
    color: var(--primary);
  }
}
</style>
