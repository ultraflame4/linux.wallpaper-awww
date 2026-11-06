<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    interface WallpaperEntry {
        name: string;
        path: string;
    }

    let {
        entry,
        imageSrc,
        onClose,
    }: {
        entry: WallpaperEntry;
        imageSrc: string;
        onClose: () => void;
    } = $props();

    let setting = $state(false);

    async function handleSet() {
        setting = true;
        try {
            await invoke("set_wallpaper", { path: entry.path });
            onClose();
        } catch (_) {
            setting = false;
        }
    }

    function handleOpenFolder() {
        invoke("open_in_folder", { path: entry.path });
    }

    function handleBackdropClick(e: MouseEvent) {
        if (e.target === e.currentTarget) onClose();
    }
</script>

<div
    class="fixed inset-0 z-50 bg-black/80 flex items-center justify-center p-8"
    role="presentation"
    onclick={handleBackdropClick}
    onkeydown={(e: KeyboardEvent) => e.key === "Escape" && onClose()}
>
    <div
        class="relative size-full flex flex-col bg-neutral-850 rounded-xl overflow-hidden shadow-2xl"
    >
        <!-- Image -->
        <div class="grow overflow-hidden p-4">
            <img
                src={imageSrc}
                alt={entry.name}
                class="size-full object-cover rounded-lg"
            />
        </div>

        <!-- Bottom bar -->
        <div
            class="flex items-center justify-between px-5 py-3 border-t border-neutral-750 bg-neutral-850 shrink-0"
        >
            <p class="text-sm text-neutral-300 truncate mr-4">{entry.name}</p>
            <div class="flex items-center gap-2 shrink-0">
                <button
                    onclick={handleOpenFolder}
                    class="px-3 py-1.5 text-xs font-medium text-neutral-300 bg-neutral-750 hover:bg-neutral-650 rounded-lg transition-colors"
                >
                    Open in folder
                </button>
                <button
                    onclick={handleSet}
                    disabled={setting}
                    class="px-4 py-1.5 text-xs font-medium text-white bg-emerald-600 hover:bg-emerald-500 disabled:opacity-50 rounded-lg transition-colors"
                >
                    {setting ? "Setting..." : "Set wallpaper"}
                </button>
                <button
                    onclick={onClose}
                    class="px-3 py-1.5 text-xs font-medium text-neutral-400 hover:text-neutral-200 hover:bg-neutral-750 rounded-lg transition-colors"
                >
                    Close
                </button>
            </div>
        </div>
    </div>
</div>
