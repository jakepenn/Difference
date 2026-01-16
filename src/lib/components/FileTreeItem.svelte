<script lang="ts">
  import type { FileTreeNode } from '../types';
  import { selectedFile, currentDiff, repoPath, baseBranch, isLoading } from '../stores/diff';
  import { getFileDiff } from '../tauri';
  import { cn } from '$lib/utils';

  export let node: FileTreeNode;
  export let depth: number;

  let expanded = node.expanded ?? true;

  async function handleClick() {
    if (node.isDirectory) {
      expanded = !expanded;
    } else if (node.file) {
      $selectedFile = node.path;
      $isLoading = true;
      try {
        const diff = await getFileDiff($repoPath, node.path, $baseBranch);
        $currentDiff = diff;
      } catch (e) {
        console.error('Failed to load diff:', e);
      } finally {
        $isLoading = false;
      }
    }
  }

  function getStatusColor(status: string): string {
    switch (status) {
      case 'added': return 'text-green-400';
      case 'deleted': return 'text-red-400';
      case 'modified': return 'text-yellow-400';
      case 'renamed': return 'text-blue-400';
      default: return 'text-muted-foreground';
    }
  }

  function getStatusIcon(status: string): string {
    switch (status) {
      case 'added': return '+';
      case 'deleted': return '−';
      case 'modified': return '~';
      case 'renamed': return '>';
      default: return '·';
    }
  }

  $: isSelected = $selectedFile === node.path;
  $: paddingLeft = `${depth * 10 + 8}px`;
</script>

<div>
  <button
    onclick={handleClick}
    class={cn(
      "w-full text-left px-2 py-0.5 text-xs flex items-center gap-1.5 transition-colors",
      "hover:bg-accent",
      isSelected ? "bg-accent text-accent-foreground" : "text-foreground/80"
    )}
    style="padding-left: {paddingLeft}"
  >
    {#if node.isDirectory}
      <span class="text-muted-foreground w-3 text-center text-[10px]">
        {expanded ? '▾' : '▸'}
      </span>
      <span class="truncate text-muted-foreground">{node.name}/</span>
    {:else if node.file}
      <span class="w-3 text-center font-bold {getStatusColor(node.file.status)}">
        {getStatusIcon(node.file.status)}
      </span>
      <span class="truncate flex-1">{node.name}</span>
      {#if node.file.additions > 0 || node.file.deletions > 0}
        <span class="text-[10px] text-muted-foreground tabular-nums">
          <span class="text-green-400/70">+{node.file.additions}</span>
          <span class="text-red-400/70">-{node.file.deletions}</span>
        </span>
      {/if}
    {/if}
  </button>

  {#if node.isDirectory && expanded}
    {#each node.children as child}
      <svelte:self node={child} depth={depth + 1} />
    {/each}
  {/if}
</div>
