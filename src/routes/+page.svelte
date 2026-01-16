<script lang="ts">
  import Header from '$lib/components/Header.svelte';
  import FileTree from '$lib/components/FileTree.svelte';
  import DiffViewer from '$lib/components/DiffViewer.svelte';
  import StatusBar from '$lib/components/StatusBar.svelte';
  import { repoInfo } from '$lib/stores/diff';
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
        <p class="text-muted-foreground/40 text-[10px] uppercase tracking-widest">open repository to start</p>
      </div>
    </div>
  {/if}

  <StatusBar />
</div>
