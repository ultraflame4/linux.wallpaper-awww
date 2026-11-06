<script lang="ts">
    interface WallpaperEntry {
        name: string;
        path: string;
    }

    let {
        wallpapers,
        currentWallpaper,
        loadImage,
        onPreview,
    }: {
        wallpapers: WallpaperEntry[];
        currentWallpaper: string | null;
        loadImage: (path: string) => Promise<string>;
        onPreview: (entry: WallpaperEntry) => void;
    } = $props();

    let images = $state<Record<string, string>>({});
    let pending = $state<Set<string>>(new Set());

    async function fetch(path: string) {
        if (images[path] || pending.has(path)) return;
        pending = new Set([...pending, path]);
        try {
            images = { ...images, [path]: await loadImage(path) };
        } finally {
            pending = new Set([...pending].filter((p) => p !== path));
        }
    }

    $effect(() => {
        for (const e of wallpapers) fetch(e.path);
    });
</script>

{#if wallpapers.length === 0}
    <div class="flex-1 flex items-center justify-center">
        <div class="text-center">
            <svg
                class="w-12 h-12 text-neutral-650 mx-auto mb-3"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                stroke-width="1"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M2.25 15.75l5.159-5.159a2.25 2.25 0 013.182 0l5.159 5.159m-1.5-1.5l1.409-1.409a2.25 2.25 0 013.182 0l2.909 2.909M3.75 21h16.5A2.25 2.25 0 0022.5 18.75V5.25A2.25 2.25 0 0020.25 3H3.75A2.25 2.25 0 001.5 5.25v13.5A2.25 2.25 0 003.75 21z"
                />
            </svg>
            <p class="text-neutral-500 text-sm">No wallpapers found</p>
            <p class="text-neutral-600 text-xs mt-1">
                Add images to ~/Pictures/Wallpapers or download below
            </p>
        </div>
    </div>
{:else}
    <div class="flex-1 overflow-y-auto p-4">
        <div
            class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-3"
        >
            {#each wallpapers as entry (entry.path)}
                {@const isCurrent = currentWallpaper === entry.path}
                {@const src = images[entry.path]}
                <button
                    class="group relative aspect-16/10 rounded-lg overflow-hidden border-2 transition-all duration-150 cursor-pointer
            {isCurrent
                        ? 'border-emerald-500/70 ring-1 ring-emerald-500/30'
                        : 'border-neutral-750 hover:border-neutral-550'}"
                    onclick={() => onPreview(entry)}
                >
                    {#if src}
                        <img
                            {src}
                            alt={entry.name}
                            class="w-full h-full object-cover"
                        />
                    {:else}
                        <div
                            class="w-full h-full bg-neutral-800 flex items-center justify-center"
                        >
                            <div
                                class="w-5 h-5 border-2 border-neutral-600 border-t-neutral-400 rounded-full animate-spin"
                            ></div>
                        </div>
                    {/if}

                    <div
                        class="absolute inset-0 bg-black/0 group-hover:bg-black/20 transition-colors"
                    ></div>

                    {#if isCurrent}
                        <div
                            class="absolute top-1.5 left-1.5 bg-emerald-500 text-white text-[10px] font-medium px-1.5 py-0.5 rounded"
                        >
                            Active
                        </div>
                    {/if}

                    <div
                        class="absolute bottom-0 left-0 right-0 p-1.5 bg-gradient-to-t from-black/70 to-transparent opacity-0 group-hover:opacity-100 transition-opacity"
                    >
                        <p
                            class="text-[11px] text-white truncate leading-tight"
                        >
                            {entry.name}
                        </p>
                    </div>
                </button>
            {/each}
        </div>
    </div>
{/if}
