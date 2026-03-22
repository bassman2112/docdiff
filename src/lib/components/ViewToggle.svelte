<script lang="ts">
  import type { ViewMode } from '$lib/types';

  let {
    mode = $bindable<ViewMode>('side-by-side'),
    fullFile = $bindable(false),
  }: {
    mode: ViewMode;
    fullFile: boolean;
  } = $props();
</script>

<div class="flex items-center gap-3">
  <div class="inline-flex rounded-lg p-1" style="background: var(--dd-toggle-bg);">
    <button
      onclick={() => (mode = 'side-by-side')}
      class="rounded-md px-3 py-1.5 text-sm font-medium transition-colors cursor-pointer"
      style="background: {mode === 'side-by-side' ? 'var(--dd-toggle-active)' : 'transparent'};
             color: {mode === 'side-by-side' ? 'var(--dd-toggle-text-active)' : 'var(--dd-toggle-text)'};"
    >
      Side by Side
    </button>
    <button
      onclick={() => (mode = 'line-by-line')}
      class="rounded-md px-3 py-1.5 text-sm font-medium transition-colors cursor-pointer"
      style="background: {mode === 'line-by-line' ? 'var(--dd-toggle-active)' : 'transparent'};
             color: {mode === 'line-by-line' ? 'var(--dd-toggle-text-active)' : 'var(--dd-toggle-text)'};"
    >
      Unified
    </button>
  </div>

  <button
    onclick={() => (fullFile = !fullFile)}
    class="flex items-center gap-1.5 rounded-lg px-3 py-1.5 text-sm font-medium transition-colors cursor-pointer"
    style="background: {fullFile ? 'color-mix(in srgb, var(--dd-accent) 20%, transparent)' : 'var(--dd-toggle-bg)'};
           color: {fullFile ? 'var(--dd-accent)' : 'var(--dd-toggle-text)'};"
    title={fullFile ? 'Show changes only' : 'Show full file'}
  >
    <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 3.75v4.5m0-4.5h4.5m-4.5 0L9 9M3.75 20.25v-4.5m0 4.5h4.5m-4.5 0L9 15M20.25 3.75h-4.5m4.5 0v4.5m0-4.5L15 9m5.25 11.25h-4.5m4.5 0v-4.5m0 4.5L15 15" />
    </svg>
    {fullFile ? 'Full File' : 'Changes Only'}
  </button>
</div>
