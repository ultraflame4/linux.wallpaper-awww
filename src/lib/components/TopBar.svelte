<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

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
    <svg class="w-5 h-5 text-neutral-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
      <path stroke-linecap="round" stroke-linejoin="round"
        d="M2.25 15.75l5.159-5.159a2.25 2.25 0 013.182 0l5.159 5.159m-1.5-1.5l1.409-1.409a2.25 2.25 0 013.182 0l2.909 2.909M3.75 21h16.5A2.25 2.25 0 0022.5 18.75V5.25A2.25 2.25 0 0020.25 3H3.75A2.25 2.25 0 001.5 5.25v13.5A2.25 2.25 0 003.75 21z" />
    </svg>
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
      <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
        <path stroke-linecap="round" stroke-linejoin="round"
          d="M12 21a9.004 9.004 0 008.716-6.747M12 21a9.004 9.004 0 01-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 017.843 4.582M12 3a8.997 8.997 0 00-7.843 4.582m15.686 0A11.953 11.953 0 0112 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0121 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0112 16.5c-3.162 0-6.133-.815-8.716-2.247m0 0A9.015 9.015 0 013 12c0-1.605.42-3.113 1.157-4.418" />
      </svg>
      From Web
    </button>
    <button
      onclick={openFolder}
      class="px-3 py-1.5 text-xs font-medium text-neutral-400 hover:text-neutral-200 bg-neutral-750 hover:bg-neutral-650 rounded-lg transition-colors flex items-center gap-1.5"
    >
      <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
        <path stroke-linecap="round" stroke-linejoin="round"
          d="M3.75 9.776c.112-.017.227-.026.344-.026h15.812c.117 0 .232.009.344.026m-16.5 0a2.25 2.25 0 00-1.883 2.542l.857 6a2.25 2.25 0 002.227 1.932H19.05a2.25 2.25 0 002.227-1.932l.857-6a2.25 2.25 0 00-1.883-2.542m-16.5 0V6A2.25 2.25 0 016 3.75h3.879a1.5 1.5 0 011.06.44l2.122 2.12a1.5 1.5 0 001.06.44H18A2.25 2.25 0 0120.25 9v.776" />
      </svg>
      Open folder
    </button>
  </div>
</header>