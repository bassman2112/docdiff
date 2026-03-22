<script lang="ts">
  import DropZone from './DropZone.svelte';
  import type { FileInfo } from '$lib/types';

  let {
    beforeFile = $bindable<FileInfo | null>(null),
    afterFile = $bindable<FileInfo | null>(null),
    oncompare,
  }: {
    beforeFile: FileInfo | null;
    afterFile: FileInfo | null;
    oncompare: () => void;
  } = $props();

  let canCompare = $derived(beforeFile !== null && afterFile !== null);

  function swap() {
    const tmp = beforeFile;
    beforeFile = afterFile;
    afterFile = tmp;
  }
</script>

<div class="flex flex-col items-center gap-6 w-full max-w-3xl mx-auto">
  <div class="flex items-center gap-4 w-full">
    <div class="flex-1">
      <DropZone label="Before" bind:file={beforeFile} />
    </div>

    <button
      onclick={swap}
      class="flex-shrink-0 rounded-full p-2 transition-colors cursor-pointer"
      style="color: var(--dd-text-muted);"
      title="Swap files"
    >
      <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <path stroke-linecap="round" stroke-linejoin="round" d="M7.5 21L3 16.5m0 0L7.5 12M3 16.5h13.5m0-13.5L21 7.5m0 0L16.5 12M21 7.5H7.5" />
      </svg>
    </button>

    <div class="flex-1">
      <DropZone label="After" bind:file={afterFile} />
    </div>
  </div>

  <button
    onclick={oncompare}
    disabled={!canCompare}
    class="rounded-xl px-8 py-3 text-base font-semibold shadow-sm transition-colors cursor-pointer"
    style="background: {canCompare ? 'var(--dd-accent)' : 'var(--dd-toggle-bg)'};
           color: {canCompare ? 'var(--dd-accent-text)' : 'var(--dd-text-muted)'};
           {canCompare ? '' : 'cursor: not-allowed;'}"
  >
    Compare Documents
  </button>
</div>
