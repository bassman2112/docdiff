<script lang="ts">
  import { computeDiffHtml, areIdentical } from '$lib/services/diffService';
  import ViewToggle from './ViewToggle.svelte';
  import ThemeToggle from './ThemeToggle.svelte';
  import Logo from './Logo.svelte';
  import type { FileInfo, ViewMode } from '$lib/types';

  let {
    beforeFile,
    afterFile,
    onreset,
  }: {
    beforeFile: FileInfo;
    afterFile: FileInfo;
    onreset: () => void;
  } = $props();

  let viewMode = $state<ViewMode>('side-by-side');
  let fullFile = $state(false);

  let identical = $derived(areIdentical(beforeFile.paragraphs, afterFile.paragraphs));

  let diffHtml = $derived(
    computeDiffHtml(
      beforeFile.name,
      afterFile.name,
      beforeFile.paragraphs,
      afterFile.paragraphs,
      viewMode,
      fullFile
    )
  );
</script>

<div class="flex flex-col h-full w-full">
  <!-- Toolbar -->
  <div class="flex items-center justify-between px-6 py-3" style="border-bottom: 1px solid var(--dd-border-subtle); background: var(--dd-bg-raised);">
    <div class="flex items-center gap-3 text-sm min-w-0">
      <Logo size={24} />
      <span class="truncate max-w-[200px] font-medium" style="color: var(--dd-red);">{beforeFile.name}</span>
      <svg class="h-4 w-4 flex-shrink-0" style="color: var(--dd-text-muted);" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <path stroke-linecap="round" stroke-linejoin="round" d="M13.5 4.5L21 12m0 0l-7.5 7.5M21 12H3" />
      </svg>
      <span class="truncate max-w-[200px] font-medium" style="color: var(--dd-green);">{afterFile.name}</span>
    </div>

    <div class="flex items-center gap-3">
      <ViewToggle bind:mode={viewMode} bind:fullFile />
      <ThemeToggle />
      <button
        onclick={onreset}
        class="rounded-lg px-4 py-2 text-sm font-medium cursor-pointer transition-colors"
        style="color: var(--dd-text); border: 1px solid var(--dd-btn-ring);"
      >
        New Comparison
      </button>
    </div>
  </div>

  <!-- Diff content -->
  <div class="flex-1 overflow-auto" style="background: var(--dd-bg-surface);">
    {#if identical}
      <div class="flex flex-col items-center justify-center h-full gap-3" style="color: var(--dd-text-muted);">
        <svg class="h-16 w-16" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1">
          <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <p class="text-lg font-medium" style="color: var(--dd-text-secondary);">No differences found</p>
        <p class="text-sm">These documents are identical.</p>
      </div>
    {:else}
      {@html diffHtml}
    {/if}
  </div>
</div>
