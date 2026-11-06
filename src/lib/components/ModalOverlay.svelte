<script lang="ts">
    let {
        onClose,
        children,
    }: { onClose: () => void; children: import("svelte").Snippet } = $props();

    function handleBackdropClick(e: MouseEvent) {
        if (e.target === e.currentTarget) onClose();
    }

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === "Escape") onClose();
    }
</script>

<svelte:window onkeydown={handleKeydown} />
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
    class="fixed inset-0 z-50 bg-black/80 flex items-center justify-center p-8"
    role="presentation"
    onclick={handleBackdropClick}
>
    {@render children()}
</div>
