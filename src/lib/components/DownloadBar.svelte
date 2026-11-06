<script lang="ts">
  let { onDownload }: { onDownload: (url: string) => void } = $props();

  let url = $state("");
  let downloading = $state(false);

  async function handleDownload(e: Event) {
    e.preventDefault();
    const trimmed = url.trim();
    if (!trimmed) return;

    downloading = true;
    try {
      await onDownload(trimmed);
      url = "";
    } finally {
      downloading = false;
    }
  }
</script>

<form
  onsubmit={handleDownload}
  class="flex items-center gap-2 px-4 py-3 border-t border-neutral-750 bg-neutral-850 shrink-0"
>
  <svg class="w-4 h-4 text-neutral-500 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
    <path stroke-linecap="round" stroke-linejoin="round"
      d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5M16.5 12L12 16.5m0 0L7.5 12m4.5 4.5V3" />
  </svg>
  <input
    type="url"
    placeholder="Paste image URL to download..."
    bind:value={url}
    class="flex-1 bg-neutral-750 text-sm text-neutral-200 placeholder-neutral-500 px-3 py-1.5 rounded-lg border border-neutral-650 focus:border-neutral-500 focus:outline-none transition-colors"
  />
  <button
    type="submit"
    disabled={downloading || !url.trim()}
    class="px-4 py-1.5 text-sm font-medium bg-neutral-700 hover:bg-neutral-600 disabled:opacity-40 disabled:cursor-not-allowed text-neutral-200 rounded-lg transition-colors shrink-0"
  >
    {downloading ? "..." : "Download"}
  </button>
</form>