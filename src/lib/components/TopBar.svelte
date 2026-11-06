<script lang="ts">
  let { statusMsg }: { statusMsg: string | null } = $props();
</script>

<header class="flex items-center justify-between px-5 py-3 border-b border-neutral-750 bg-neutral-850 shrink-0">
  <div class="flex items-center gap-3">
    <svg class="w-5 h-5 text-neutral-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
      <path stroke-linecap="round" stroke-linejoin="round"
        d="M2.25 15.75l5.159-5.159a2.25 2.25 0 013.182 0l5.159 5.159m-1.5-1.5l1.409-1.409a2.25 2.25 0 013.182 0l2.909 2.909M3.75 21h16.5A2.25 2.25 0 0022.5 18.75V5.25A2.25 2.25 0 0020.25 3H3.75A2.25 2.25 0 001.5 5.25v13.5A2.25 2.25 0 003.75 21z" />
    </svg>
    <h1 class="text-sm font-semibold text-neutral-200 tracking-tight">Wallpaper Switcher</h1>
  </div>

  {#if statusMsg}
    <p
      class="text-xs px-3 py-1 rounded-md {statusMsg.startsWith('Error') || statusMsg.startsWith('Download failed')
        ? 'bg-red-500/10 text-red-400'
        : 'bg-emerald-500/10 text-emerald-400'}"
    >
      {statusMsg}
    </p>
  {/if}
</header>