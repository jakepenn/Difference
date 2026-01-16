<script lang="ts">
  import { fileTree, selectedFile, summary, changedFiles } from '../stores/diff';
  import FileTreeItem from './FileTreeItem.svelte';
  import { ScrollArea } from '$lib/components/ui/scroll-area';
  import { Separator } from '$lib/components/ui/separator';
</script>

<aside class="w-64 flex-shrink-0 bg-card border-r border-border flex flex-col overflow-hidden">
  <div class="px-3 py-2 border-b border-border">
    <h2 class="text-[10px] font-medium text-muted-foreground uppercase tracking-widest">files</h2>
  </div>

  <ScrollArea class="flex-1">
    {#if $changedFiles.length === 0}
      <div class="px-3 py-8 text-center text-muted-foreground text-xs">
        no changes
      </div>
    {:else}
      <div class="py-1">
        {#each $fileTree as node}
          <FileTreeItem {node} depth={0} />
        {/each}
      </div>
    {/if}
  </ScrollArea>

  {#if $changedFiles.length > 0}
    <Separator />
    <div class="px-3 py-2 text-[10px] text-muted-foreground">
      <div class="flex items-center justify-between">
        <span>{$summary.fileCount}</span>
        <span class="tabular-nums">
          <span class="text-green-400">+{$summary.additions}</span>
          <span class="mx-0.5 text-muted-foreground/50">·</span>
          <span class="text-red-400">-{$summary.deletions}</span>
        </span>
      </div>
    </div>
  {/if}
</aside>
