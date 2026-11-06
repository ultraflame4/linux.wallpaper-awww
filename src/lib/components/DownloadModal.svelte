<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

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

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) onClose();
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="fixed inset-0 z-50 bg-black/80 flex items-center justify-center p-8"
  role="presentation"
  onclick={handleBackdropClick}
  onkeydown={(e: KeyboardEvent) => e.key === "Escape" && onClose()}
>
  <div class="relative w-full bg-neutral-850 rounded-xl overflow-hidden shadow-2xl flex flex-col">
    <!-- Header -->
    <div class="flex items-center justify-between px-5 py-3 border-b border-neutral-750">
      <h2 class="text-sm font-semibold text-neutral-200">Download from Web</h2>
      <button
        onclick={onClose}
        aria-label="Close"
        class="text-neutral-500 hover:text-neutral-300 transition-colors"
      >
        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
        </svg>
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
      <div class="aspect-[16/10] rounded-lg bg-neutral-900 overflow-hidden flex items-center justify-center">
        {#if previewLoading}
          <div class="w-6 h-6 border-2 border-neutral-600 border-t-neutral-400 rounded-full animate-spin"></div>
        {:else if previewError}
          <p class="text-xs text-red-400 px-4 text-center">{previewError}</p>
        {:else if previewSrc}
          <img src={previewSrc} alt="Preview" class="w-full h-full object-contain" />
        {:else}
          <div class="text-center text-neutral-600">
            <svg class="w-10 h-10 mx-auto mb-2" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1">
              <path stroke-linecap="round" stroke-linejoin="round"
                d="M2.25 15.75l5.159-5.159a2.25 2.25 0 013.182 0l5.159 5.159m-1.5-1.5l1.409-1.409a2.25 2.25 0 013.182 0l2.909 2.909M3.75 21h16.5A2.25 2.25 0 0022.5 18.75V5.25A2.25 2.25 0 0020.25 3H3.75A2.25 2.25 0 001.5 5.25v13.5A2.25 2.25 0 003.75 21z" />
            </svg>
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
</div>
