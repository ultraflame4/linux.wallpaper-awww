<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import TopBar from "$lib/components/TopBar.svelte";
  import WallpaperGrid from "$lib/components/WallpaperGrid.svelte";
  import PreviewModal from "$lib/components/PreviewModal.svelte";
  import DownloadModal from "$lib/components/DownloadModal.svelte";

  interface WallpaperEntry {
    name: string;
    path: string;
  }

  let wallpapers = $state<WallpaperEntry[]>([]);
  let currentWallpaper = $state<string | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let statusMsg = $state<string | null>(null);

  let imageCache = $state<Record<string, string>>({});

  // Preview state
  let previewEntry = $state<WallpaperEntry | null>(null);
  let previewSrc = $state<string>("");

  // Download modal state
  let showDownload = $state(false);

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

  async function loadImage(path: string): Promise<string> {
    if (imageCache[path]) return imageCache[path];
    const dataUrl = await invoke<string>("read_image", { path });
    imageCache = { ...imageCache, [path]: dataUrl };
    return dataUrl;
  }

  async function openPreview(entry: WallpaperEntry) {
    previewEntry = entry;
    previewSrc = imageCache[entry.path] || "";
    if (!previewSrc) {
      previewSrc = await loadImage(entry.path);
    }
  }

  function closePreview() {
    previewEntry = null;
    previewSrc = "";
    invoke<string | null>("get_current_wallpaper").then((c) => {
      currentWallpaper = c;
    });
  }

  function handleDownloaded(entry: WallpaperEntry) {
    wallpapers = [...wallpapers, entry];
    statusMsg = `Downloaded: ${entry.name}`;
  }

  $effect(() => {
    loadWallpapers();
  });
</script>

<div class="flex flex-col h-full">
  <TopBar {statusMsg} onOpenDownload={() => (showDownload = true)} />

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
        {loadImage}
        onPreview={openPreview}
      />
    {/if}
  </main>
</div>

{#if previewEntry && previewSrc}
  <PreviewModal entry={previewEntry} imageSrc={previewSrc} onClose={closePreview} />
{/if}

{#if showDownload}
  <DownloadModal
    onClose={() => (showDownload = false)}
    onDownloaded={handleDownloaded}
  />
{/if}