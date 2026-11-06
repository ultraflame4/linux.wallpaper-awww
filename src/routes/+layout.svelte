<script lang="ts">
  import "../app.css";
  import { addCollection } from "@iconify/svelte";
  import phosphor from "@iconify-json/ph";

  addCollection(phosphor.icons);

  let { children } = $props();
</script>

<div class="h-full">
  {@render children()}
</div>
