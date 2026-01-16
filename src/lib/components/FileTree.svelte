<script lang="ts">
  import {
    fileTree,
    selectedFile,
    summary,
    filteredSummary,
    changedFiles,
    filteredFiles,
    fileSearch,
    showAdded,
    showModified,
    showDeleted,
    showCosmetic,
    allCollapsed,
    setHover,
    clearHover
  } from '../stores';
  import { setSearchInput } from '../keyboard';
  import FileTreeItem from './FileTreeItem.svelte';
  import { Separator } from '$lib/components/ui/separator';
  import { cn } from '$lib/utils';

  let searchEl: HTMLInputElement;
  $effect(() => setSearchInput(searchEl));

  function toggleStatus(status: 'added' | 'modified' | 'deleted' | 'cosmetic') {
    if (status === 'added') showAdded.update(v => !v);
    if (status === 'modified') showModified.update(v => !v);
    if (status === 'deleted') showDeleted.update(v => !v);
    if (status === 'cosmetic') showCosmetic.update(v => !v);
  }

  function collapseAll() {
    allCollapsed.set(true);
    // Reset after a tick to allow components to react
    setTimeout(() => allCollapsed.set(false), 50);
  }

  function expandAll() {
    // Force re-render by clearing search briefly
    const currentSearch = $fileSearch;
    fileSearch.set(currentSearch + ' ');
    setTimeout(() => fileSearch.set(currentSearch), 10);
  }

  const isFiltered = $derived(!$showAdded || !$showModified || !$showDeleted || !$showCosmetic || $fileSearch.length > 0);
  const hiddenCount = $derived($changedFiles.length - $filteredFiles.length);
</script>

<aside class="w-64 flex-shrink-0 bg-card border-r border-border flex flex-col overflow-hidden">
  <!-- Header -->
  <div class="px-3 py-2 border-b border-border flex-shrink-0 flex items-center justify-between">
    <h2 class="text-[10px] font-medium text-muted-foreground uppercase tracking-widest">files</h2>
    <div class="flex items-center gap-1">
      <button
        onclick={collapseAll}
        class="text-[10px] text-muted-foreground hover:text-foreground px-1"
        onmouseenter={() => setHover({ label: 'collapse all', description: 'collapse all directories' })}
        onmouseleave={clearHover}
      >
        ⊟
      </button>
      <button
        onclick={expandAll}
        class="text-[10px] text-muted-foreground hover:text-foreground px-1"
        onmouseenter={() => setHover({ label: 'expand all', description: 'expand all directories' })}
        onmouseleave={clearHover}
      >
        ⊞
      </button>
    </div>
  </div>

  <!-- Search -->
  <div class="px-2 py-1.5 border-b border-border flex-shrink-0">
    <input
      bind:this={searchEl}
      type="text"
      placeholder="filter files..."
      bind:value={$fileSearch}
      class="w-full bg-background border border-border rounded px-2 py-1 text-xs placeholder:text-muted-foreground/50 focus:outline-none focus:border-muted-foreground/50"
      onmouseenter={() => setHover({ label: 'search', description: 'fuzzy filter files by path' })}
      onmouseleave={clearHover}
    />
  </div>

  <!-- Status Toggles -->
  <div class="px-2 py-1.5 border-b border-border flex-shrink-0 flex items-center gap-1">
    <button
      onclick={() => toggleStatus('added')}
      class={cn(
        "flex items-center gap-1 px-1.5 py-0.5 rounded text-[10px] transition-colors",
        $showAdded
          ? "bg-green-500/10 text-green-400"
          : "bg-muted/30 text-muted-foreground/40 line-through"
      )}
      onmouseenter={() => setHover({
        label: $showAdded ? 'hide added' : 'show added',
        description: `${$summary.added} added file${$summary.added !== 1 ? 's' : ''}`
      })}
      onmouseleave={clearHover}
    >
      <span class="font-bold">+</span>
      <span>{$summary.added}</span>
    </button>

    <button
      onclick={() => toggleStatus('modified')}
      class={cn(
        "flex items-center gap-1 px-1.5 py-0.5 rounded text-[10px] transition-colors",
        $showModified
          ? "bg-yellow-500/10 text-yellow-400"
          : "bg-muted/30 text-muted-foreground/40 line-through"
      )}
      onmouseenter={() => setHover({
        label: $showModified ? 'hide modified' : 'show modified',
        description: `${$summary.modified} modified file${$summary.modified !== 1 ? 's' : ''}`
      })}
      onmouseleave={clearHover}
    >
      <span class="font-bold">~</span>
      <span>{$summary.modified}</span>
    </button>

    <button
      onclick={() => toggleStatus('deleted')}
      class={cn(
        "flex items-center gap-1 px-1.5 py-0.5 rounded text-[10px] transition-colors",
        $showDeleted
          ? "bg-red-500/10 text-red-400"
          : "bg-muted/30 text-muted-foreground/40 line-through"
      )}
      onmouseenter={() => setHover({
        label: $showDeleted ? 'hide deleted' : 'show deleted',
        description: `${$summary.deleted} deleted file${$summary.deleted !== 1 ? 's' : ''}`
      })}
      onmouseleave={clearHover}
    >
      <span class="font-bold">−</span>
      <span>{$summary.deleted}</span>
    </button>

    {#if $summary.cosmetic > 0}
      <span class="text-muted-foreground/30">|</span>
      <button
        onclick={() => toggleStatus('cosmetic')}
        class={cn(
          "flex items-center gap-1 px-1.5 py-0.5 rounded text-[10px] transition-colors",
          $showCosmetic
            ? "bg-muted/50 text-muted-foreground"
            : "bg-muted/30 text-muted-foreground/40 line-through"
        )}
        onmouseenter={() => setHover({
          label: $showCosmetic ? 'hide cosmetic' : 'show cosmetic',
          description: `${$summary.cosmetic} cosmetic-only change${$summary.cosmetic !== 1 ? 's' : ''} (comments, whitespace)`
        })}
        onmouseleave={clearHover}
      >
        <span class="font-bold">◊</span>
        <span>{$summary.cosmetic}</span>
      </button>
    {/if}
  </div>

  <!-- File List -->
  <div class="flex-1 min-h-0 overflow-y-auto">
    {#if $filteredFiles.length === 0}
      <div class="px-3 py-8 text-center text-muted-foreground text-xs">
        {#if $changedFiles.length === 0}
          no changes
        {:else}
          no matches
        {/if}
      </div>
    {:else}
      <div class="py-1">
        {#each $fileTree as node (node.path)}
          <FileTreeItem {node} depth={0} />
        {/each}
      </div>
    {/if}
  </div>

  <!-- Footer Summary -->
  {#if $changedFiles.length > 0}
    <Separator class="flex-shrink-0" />
    <div class="px-3 py-2 text-[10px] text-muted-foreground flex-shrink-0">
      <div class="flex items-center justify-between">
        <span>
          {$filteredSummary.fileCount}
          {#if isFiltered}
            <span class="text-muted-foreground/50">/ {$summary.fileCount}</span>
          {/if}
        </span>
        <span class="tabular-nums">
          <span class="text-green-400">+{$filteredSummary.additions}</span>
          <span class="mx-0.5 text-muted-foreground/50">·</span>
          <span class="text-red-400">-{$filteredSummary.deletions}</span>
        </span>
      </div>
    </div>
  {/if}
</aside>
