<script lang="ts">
  import type { FileTreeNode } from '../types';
  import { selectedFile, currentDiff, repoPath, baseBranch, isLoading, allCollapsed, setHover, clearHover } from '../stores';
  import { getFileDiff } from '../tauri';
  import { cn } from '$lib/utils';
  import FileTreeItem from './FileTreeItem.svelte';

  interface Props {
    node: FileTreeNode;
    depth: number;
  }

  let { node, depth }: Props = $props();

  // Initialize expanded state - intentionally captures initial value only
  let expanded = $state((() => node.expanded ?? true)());

  // React to collapse all trigger
  $effect(() => {
    if ($allCollapsed && node.isDirectory) {
      expanded = false;
    }
  });

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

  function getStatusLabel(status: string): string {
    switch (status) {
      case 'added': return 'new file';
      case 'deleted': return 'deleted';
      case 'modified': return 'modified';
      case 'renamed': return 'renamed';
      default: return 'unchanged';
    }
  }

  function handleMouseEnter() {
    if (node.isDirectory) {
      setHover({
        label: node.path,
        description: `${node.children.length} item${node.children.length !== 1 ? 's' : ''}`
      });
    } else if (node.file) {
      const stats = node.file.additions > 0 || node.file.deletions > 0
        ? `+${node.file.additions} -${node.file.deletions}`
        : '';
      const cosmetic = node.file.is_cosmetic ? ' (cosmetic)' : '';
      setHover({
        label: node.path,
        description: `${getStatusLabel(node.file.status)}${cosmetic}${stats ? ' · ' + stats : ''}`
      });
    }
  }

  const isSelected = $derived($selectedFile === node.path);
  const paddingLeft = $derived(`${depth * 10 + 8}px`);
</script>

<div>
  <button
    onclick={handleClick}
    onmouseenter={handleMouseEnter}
    onmouseleave={clearHover}
    data-file-path={node.file ? node.path : undefined}
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
      <span class={node.file.is_cosmetic ? "truncate flex-1 opacity-50" : "truncate flex-1"}>{node.name}</span>
      {#if node.file.is_cosmetic}
        <span class="text-[10px] text-muted-foreground/50" title="cosmetic">◊</span>
      {/if}
      {#if node.file.additions > 0 || node.file.deletions > 0}
        <span class="text-[10px] text-muted-foreground tabular-nums">
          <span class="text-green-400/70">+{node.file.additions}</span>
          <span class="text-red-400/70">-{node.file.deletions}</span>
        </span>
      {/if}
    {/if}
  </button>

  {#if node.isDirectory && expanded}
    {#each node.children as child (child.path)}
      <FileTreeItem node={child} depth={depth + 1} />
    {/each}
  {/if}
</div>
