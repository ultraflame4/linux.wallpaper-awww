<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Icon from "@iconify/svelte";
  import ModalOverlay from "./ModalOverlay.svelte";

  interface WallpaperEntry {
    name: string;
    path: string;
  }

  let {
    onClose,
    onDownloaded,
  }: {
    onClose: () => void;
    onDownloaded: (entry: WallpaperEntry) => void;
  } = $props();

  let url = $state("");
  let previewSrc = $state<string | null>(null);
  let previewLoading = $state(false);
  let previewError = $state<string | null>(null);
  let downloading = $state(false);
  let statusMsg = $state<string | null>(null);

  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  function handleInput(value: string) {
    url = value;
    previewError = null;

    if (debounceTimer) clearTimeout(debounceTimer);
    if (!value.trim()) {
      previewSrc = null;
      return;
    }

    debounceTimer = setTimeout(async () => {
      previewLoading = true;
      try {
        previewSrc = await invoke<string>("preview_url", { url: value.trim() });
      } catch (e) {
        previewError = String(e);
        previewSrc = null;
      } finally {
        previewLoading = false;
      }
    }, 500);
  }

  async function handleDownload() {
    downloading = true;
    statusMsg = null;
    try {
      const entry = await invoke<WallpaperEntry>("download_wallpaper", { url: url.trim() });
      statusMsg = "Downloaded!";
      onDownloaded(entry);
      setTimeout(onClose, 800);
    } catch (e) {
      statusMsg = `Error: ${e}`;
    } finally {
      downloading = false;
    }
  }
</script>

<ModalOverlay {onClose}>
  {#snippet children()}
    <div class="relative w-full max-w-lg bg-neutral-850 rounded-xl overflow-hidden shadow-2xl flex flex-col">
      <!-- Header -->
      <div class="flex items-center justify-between px-5 py-3 border-b border-neutral-750">
        <h2 class="text-sm font-semibold text-neutral-200">Download from Web</h2>
        <button
          onclick={onClose}
          aria-label="Close"
          class="text-neutral-500 hover:text-neutral-300 transition-colors"
        >
          <Icon icon="ph:x" class="w-4 h-4" />
        </button>
      </div>

      <!-- URL input -->
      <div class="px-5 pt-4">
        <input
          type="url"
          placeholder="Paste image URL..."
          value={url}
          oninput={(e) => handleInput(e.currentTarget.value)}
          class="w-full bg-neutral-750 text-sm text-neutral-200 placeholder-neutral-500 px-3 py-2 rounded-lg border border-neutral-650 focus:border-neutral-500 focus:outline-none transition-colors"
        />
      </div>

      <!-- Preview area -->
      <div class="px-5 py-4">
        <div class="aspect-16/10 rounded-lg bg-neutral-900 overflow-hidden flex items-center justify-center">
          {#if previewLoading}
            <div class="w-6 h-6 border-2 border-neutral-600 border-t-neutral-400 rounded-full animate-spin"></div>
          {:else if previewError}
            <p class="text-xs text-red-400 px-4 text-center">{previewError}</p>
          {:else if previewSrc}
            <img src={previewSrc} alt="Preview" class="w-full h-full object-contain" />
          {:else}
            <div class="text-center text-neutral-600">
              <Icon icon="ph:image" class="w-10 h-10 mx-auto mb-2" />
              <p class="text-xs">Enter a URL to preview</p>
            </div>
          {/if}
        </div>
      </div>

      <!-- Bottom bar -->
      <div class="flex items-center justify-between px-5 py-3 border-t border-neutral-750">
        <div class="flex-1">
          {#if statusMsg}
            <p
              class="text-xs {statusMsg.startsWith('Error')
                ? 'text-red-400'
                : 'text-emerald-400'}"
            >
              {statusMsg}
            </p>
          {/if}
        </div>
        <button
          onclick={handleDownload}
          disabled={downloading || !previewSrc}
          class="px-4 py-1.5 text-xs font-medium text-white bg-emerald-600 hover:bg-emerald-500 disabled:opacity-40 disabled:cursor-not-allowed rounded-lg transition-colors"
        >
          {downloading ? "Downloading..." : "Download"}
        </button>
      </div>
    </div>
  {/snippet}
</ModalOverlay>