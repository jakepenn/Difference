<script lang="ts">
  import Header from '$lib/components/Header.svelte';
  import FileTree from '$lib/components/FileTree.svelte';
  import DiffViewer from '$lib/components/DiffViewer.svelte';
  import StatusBar from '$lib/components/StatusBar.svelte';
  import HelpModal from '$lib/components/HelpModal.svelte';
  import { repoInfo } from '$lib/stores';
</script>

<div class="h-screen flex flex-col bg-background text-foreground">
  <Header />

  {#if $repoInfo}
    <div class="flex-1 flex overflow-hidden">
      <FileTree />
      <DiffViewer />
    </div>
  {:else}
    <div class="flex-1 flex items-center justify-center">
      <div class="text-center">
        <div class="text-4xl mb-4 text-muted-foreground/20 font-bold tracking-tighter">///</div>
        <h2 class="text-sm font-bold text-foreground mb-1 uppercase tracking-wide">Difference</h2>
        <p class="text-muted-foreground text-xs mb-6">git branch diff viewer</p>

        <div class="flex items-center justify-center gap-3 mb-6">
          <kbd class="px-2 py-1 bg-muted rounded text-xs">o</kbd>
          <span class="text-muted-foreground text-xs">open repository</span>
        </div>

        <div class="flex items-center justify-center gap-4 text-[10px] text-muted-foreground/50">
          <span><kbd class="px-1 py-0.5 bg-muted/50 rounded mr-1">j</kbd><kbd class="px-1 py-0.5 bg-muted/50 rounded">k</kbd> navigate</span>
          <span><kbd class="px-1 py-0.5 bg-muted/50 rounded mr-1">/</kbd> search</span>
          <span><kbd class="px-1 py-0.5 bg-muted/50 rounded mr-1">v</kbd> toggle view</span>
        </div>

        <p class="text-muted-foreground/30 text-[10px] mt-6">
          press <kbd class="px-1 py-0.5 bg-muted/30 rounded">?</kbd> for all shortcuts
        </p>
      </div>
    </div>
  {/if}

  <HelpModal />
  <StatusBar />
</div>
