<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Icon from "@iconify/svelte";

  let {
    statusMsg,
    onOpenDownload,
  }: {
    statusMsg: string | null;
    onOpenDownload: () => void;
  } = $props();

  function openFolder() {
    invoke("open_wallpapers_folder");
  }
</script>

<header class="flex items-center justify-between px-5 py-3 border-b border-neutral-750 bg-neutral-850 shrink-0">
  <div class="flex items-center gap-3">
    <Icon icon="ph:image" class="w-5 h-5 text-neutral-400" />
    <h1 class="text-sm font-semibold text-neutral-200 tracking-tight">Wallpaper Switcher</h1>
  </div>

  <div class="flex items-center gap-2">
    {#if statusMsg}
      <p
        class="text-xs px-3 py-1 rounded-md {statusMsg.startsWith('Error') || statusMsg.startsWith('Download failed')
          ? 'bg-red-500/10 text-red-400'
          : 'bg-emerald-500/10 text-emerald-400'}"
      >
        {statusMsg}
      </p>
    {/if}
    <button
      onclick={onOpenDownload}
      class="px-3 py-1.5 text-xs font-medium text-neutral-300 bg-neutral-750 hover:bg-neutral-650 rounded-lg transition-colors flex items-center gap-1.5"
    >
      <Icon icon="ph:globe" class="w-3.5 h-3.5" />
      From Web
    </button>
    <button
      onclick={openFolder}
      class="px-3 py-1.5 text-xs font-medium text-neutral-400 hover:text-neutral-200 bg-neutral-750 hover:bg-neutral-650 rounded-lg transition-colors flex items-center gap-1.5"
    >
      <Icon icon="ph:folder-open" class="w-3.5 h-3.5" />
      Open folder
    </button>
  </div>
</header>
