<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { init } from '$lib/theme.svelte';
  import FileUploadPanel from '$lib/components/FileUploadPanel.svelte';
  import DiffViewer from '$lib/components/DiffViewer.svelte';
  import Logo from '$lib/components/Logo.svelte';
  import ThemeToggle from '$lib/components/ThemeToggle.svelte';
  import type { AppState, FileInfo } from '$lib/types';

  let appState = $state<AppState>('upload');
  let beforeFile = $state<FileInfo | null>(null);
  let afterFile = $state<FileInfo | null>(null);

  onMount(() => init());

  function compare() {
    if (beforeFile && afterFile) {
      appState = 'diff';
    }
  }

  function reset() {
    appState = 'upload';
    beforeFile = null;
    afterFile = null;
  }
</script>

<div class="flex flex-col h-screen" style="background: var(--dd-bg);">
  {#if appState === 'upload'}
    <div class="absolute top-4 right-4">
      <ThemeToggle />
    </div>
    <div class="flex flex-1 flex-col items-center justify-center p-8">
      <div class="mb-8 text-center flex flex-col items-center gap-3">
        <Logo size={56} />
        <h1 class="text-3xl font-bold" style="color: var(--dd-text);">Doc Diff</h1>
        <p style="color: var(--dd-text-muted);">Compare two documents and see what changed</p>
      </div>
      <FileUploadPanel
        bind:beforeFile
        bind:afterFile
        oncompare={compare}
      />
    </div>
  {:else if appState === 'diff' && beforeFile && afterFile}
    <DiffViewer
      {beforeFile}
      {afterFile}
      onreset={reset}
    />
  {/if}
</div>
