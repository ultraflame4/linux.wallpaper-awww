<script lang="ts">
  import "@fontsource-variable/roboto";
  import "@fontsource-variable/open-sans";
  import "../app.css";
  import { addCollection } from "@iconify/svelte";
  import {icons as phosphor} from "@iconify-json/ph";

  addCollection(phosphor);

  let { children } = $props();
</script>

<div class="h-full">
  {@render children()}
</div>
