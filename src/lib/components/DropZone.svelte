<script lang="ts">
  import { getCurrentWebview } from '@tauri-apps/api/webview';
  import { pickDocFile, extractText } from '$lib/services/fileService';
  import { onMount, onDestroy } from 'svelte';
  import type { FileInfo } from '$lib/types';

  let {
    label,
    file = $bindable<FileInfo | null>(null),
  }: {
    label: string;
    file: FileInfo | null;
  } = $props();

  let dragOver = $state(false);
  let loading = $state(false);
  let error = $state<string | null>(null);
  let dropZoneEl: HTMLDivElement;
  let unlisten: (() => void) | null = null;

  let lineCount = $derived(file ? file.paragraphs.length : 0);
  let charCount = $derived(file ? file.paragraphs.reduce((sum, p) => sum + p.length, 0) : 0);

  function formatCount(n: number): string {
    if (n >= 1000) return (n / 1000).toFixed(1).replace(/\.0$/, '') + 'k';
    return n.toString();
  }

  async function loadFile(filePath: string) {
    const name = filePath.split('/').pop() ?? filePath.split('\\').pop() ?? filePath;
    loading = true;
    error = null;
    try {
      const paragraphs = await extractText(filePath);
      file = { path: filePath, name, paragraphs };
    } catch (e) {
      error = String(e);
      file = null;
    } finally {
      loading = false;
    }
  }

  async function handleBrowse() {
    const path = await pickDocFile();
    if (path) await loadFile(path);
  }

  function clear() {
    file = null;
    error = null;
  }

  onMount(async () => {
    const webview = getCurrentWebview();
    unlisten = await webview.onDragDropEvent((event) => {
      if (!dropZoneEl) return;
      const rect = dropZoneEl.getBoundingClientRect();

      if (event.payload.type === 'over') {
        const pos = event.payload.position;
        const inside =
          pos.x >= rect.left && pos.x <= rect.right &&
          pos.y >= rect.top && pos.y <= rect.bottom;
        dragOver = inside;
      } else if (event.payload.type === 'drop') {
        const pos = event.payload.position;
        const inside =
          pos.x >= rect.left && pos.x <= rect.right &&
          pos.y >= rect.top && pos.y <= rect.bottom;
        dragOver = false;
        if (inside && event.payload.paths.length > 0) {
          loadFile(event.payload.paths[0]);
        }
      } else if (event.payload.type === 'leave') {
        dragOver = false;
      }
    });
  });

  onDestroy(() => {
    unlisten?.();
  });
</script>

<div
  bind:this={dropZoneEl}
  class="flex flex-col items-center justify-center rounded-xl border-2 border-dashed p-8 transition-colors min-h-[220px]"
  style="border-color: {dragOver ? 'var(--dd-drop-active-border)' : file ? 'var(--dd-drop-ok-border)' : error ? 'var(--dd-drop-err-border)' : 'var(--dd-drop-border)'};
         background: {dragOver ? 'var(--dd-drop-active-bg)' : file ? 'var(--dd-drop-ok-bg)' : error ? 'var(--dd-drop-err-bg)' : 'var(--dd-bg-raised)'};"
>
  <p class="text-xs font-semibold uppercase tracking-wider mb-3" style="color: var(--dd-text-muted);">{label}</p>

  {#if loading}
    <div class="flex flex-col items-center gap-2">
      <svg class="animate-spin h-8 w-8" style="color: var(--dd-blue);" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
      </svg>
      <span class="text-sm" style="color: var(--dd-text-muted);">Reading file...</span>
    </div>
  {:else if file}
    <div class="flex flex-col items-center gap-2">
      <svg class="h-8 w-8" style="color: var(--dd-green);" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <span class="text-sm font-medium truncate max-w-[200px]" style="color: var(--dd-text);">{file.name}</span>
      <span class="text-xs" style="color: var(--dd-text-muted);">{formatCount(lineCount)} lines &middot; {formatCount(charCount)} chars</span>
      <button onclick={clear} class="text-xs cursor-pointer mt-1 hover:opacity-80" style="color: var(--dd-red);">Remove</button>
    </div>
  {:else if error}
    <div class="flex flex-col items-center gap-2">
      <svg class="h-8 w-8" style="color: var(--dd-red);" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4.5c-.77-.833-2.694-.833-3.464 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z" />
      </svg>
      <span class="text-sm text-center max-w-[250px]" style="color: var(--dd-red);">{error}</span>
      <button onclick={clear} class="text-xs cursor-pointer mt-1" style="color: var(--dd-text-muted);">Try again</button>
    </div>
  {:else}
    <div class="flex flex-col items-center gap-3">
      <svg class="h-10 w-10" style="color: var(--dd-text-muted);" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
        <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m6.75 12l-3-3m0 0l-3 3m3-3v6m-1.5-15H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z" />
      </svg>
      <p class="text-sm" style="color: var(--dd-text-muted);">Drop a file here</p>
      <button
        onclick={handleBrowse}
        class="rounded-lg px-4 py-2 text-sm font-medium shadow-sm cursor-pointer transition-colors"
        style="background: var(--dd-bg-input); color: var(--dd-text); border: 1px solid var(--dd-btn-ring);"
      >
        Browse
      </button>
    </div>
  {/if}
</div>
