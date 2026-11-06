<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import TopBar from "$lib/components/TopBar.svelte";
  import WallpaperGrid from "$lib/components/WallpaperGrid.svelte";
  import DownloadBar from "$lib/components/DownloadBar.svelte";

  interface WallpaperEntry {
    name: string;
    path: string;
  }

  let wallpapers = $state<WallpaperEntry[]>([]);
  let currentWallpaper = $state<string | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let statusMsg = $state<string | null>(null);

  async function loadWallpapers() {
    loading = true;
    error = null;
    try {
      wallpapers = await invoke<WallpaperEntry[]>("list_wallpapers");
      const current = await invoke<string | null>("get_current_wallpaper");
      currentWallpaper = current;
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function setWallpaper(entry: WallpaperEntry) {
    statusMsg = null;
    try {
      const msg = await invoke<string>("set_wallpaper", { path: entry.path });
      currentWallpaper = entry.path;
      statusMsg = msg;
    } catch (e) {
      statusMsg = `Error: ${e}`;
    }
  }

  async function downloadWallpaper(url: string) {
    statusMsg = null;
    try {
      const entry = await invoke<WallpaperEntry>("download_wallpaper", { url });
      wallpapers = [...wallpapers, entry];
      statusMsg = `Downloaded: ${entry.name}`;
    } catch (e) {
      statusMsg = `Download failed: ${e}`;
    }
  }

  function srcFor(path: string): string {
    return convertFileSrc(path);
  }

  $effect(() => {
    loadWallpapers();
  });
</script>

<div class="flex flex-col h-screen">
  <TopBar {statusMsg} />

  <main class="flex-1 overflow-hidden flex flex-col">
    {#if loading}
      <div class="flex-1 flex items-center justify-center">
        <p class="text-neutral-500 text-sm">Loading wallpapers...</p>
      </div>
    {:else if error}
      <div class="flex-1 flex items-center justify-center">
        <div class="text-center">
          <p class="text-red-400 text-sm mb-2">Failed to load wallpapers</p>
          <p class="text-neutral-500 text-xs">{error}</p>
          <button
            class="mt-4 px-4 py-1.5 text-sm bg-neutral-750 hover:bg-neutral-650 rounded-lg transition-colors"
            onclick={loadWallpapers}
          >
            Retry
          </button>
        </div>
      </div>
    {:else}
      <WallpaperGrid
        {wallpapers}
        {currentWallpaper}
        {srcFor}
        onSelect={setWallpaper}
      />
    {/if}
  </main>

  <DownloadBar onDownload={downloadWallpaper} />
</div>