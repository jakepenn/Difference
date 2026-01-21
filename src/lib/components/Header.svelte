<script lang="ts">
  import { repoPath, repoInfo, baseBranch, isLoading, changedFiles, selectedFile, currentDiff, error, setHover, clearHover } from '../stores';
  import { selectFolder, getRepoInfo, getChangedFiles, watchRepo } from '../tauri';
  import { Button } from '$lib/components/ui/button';
  import { Badge } from '$lib/components/ui/badge';
  import BranchSelector from './BranchSelector.svelte';

  async function handleSelectFolder() {
    const folder = await selectFolder();
    if (folder) {
      $repoPath = folder;
      await loadRepo();
    }
  }

  async function loadRepo() {
    if (!$repoPath) return;

    $isLoading = true;
    $error = null;

    try {
      const info = await getRepoInfo($repoPath);
      $repoInfo = info;
      $baseBranch = info.default_base;
      await refreshFiles();
      // Start watching for git changes
      await watchRepo($repoPath);
    } catch (e) {
      $error = e as string;
    } finally {
      $isLoading = false;
    }
  }

  // Full refresh - updates branch info and files
  async function refresh() {
    if (!$repoPath) return;

    $isLoading = true;
    $error = null;

    try {
      // Refresh repo info to get current branch
      const info = await getRepoInfo($repoPath);
      $repoInfo = info;
      // Keep current base branch unless it no longer exists
      const branchExists = info.branches.some(b => b.name === $baseBranch);
      if (!branchExists) {
        $baseBranch = info.default_base;
      }
      await refreshFiles();
    } catch (e) {
      $error = e as string;
    } finally {
      $isLoading = false;
    }
  }

  // Just refresh the file list
  async function refreshFiles() {
    if (!$repoPath || !$baseBranch) return;

    const previouslySelected = $selectedFile;

    try {
      const files = await getChangedFiles($repoPath, $baseBranch);
      $changedFiles = files;

      // Preserve selection if the file still exists in the updated list
      if (previouslySelected && files.find(f => f.path === previouslySelected)) {
        const { getFileDiff } = await import('../tauri');
        $currentDiff = await getFileDiff($repoPath, previouslySelected, $baseBranch);
      } else {
        $selectedFile = null;
        $currentDiff = null;
      }
    } catch (e) {
      $error = e as string;
    }
  }

  function handleBranchChange(newBranch: string) {
    $baseBranch = newBranch;
    refreshFiles();
  }

  const localBranches = $derived($repoInfo?.branches.filter(b => !b.is_remote) ?? []);
</script>

<header class="flex items-center justify-between px-4 py-2.5 bg-card border-b border-border">
  <div class="flex items-center gap-4">
    <h1 class="text-sm font-bold tracking-tight text-foreground uppercase">Difference</h1>

    {#if $repoInfo}
      <div class="flex items-center gap-2 text-xs">
        <!-- Base branch (left) - what you're comparing against -->
        <BranchSelector
          branches={localBranches}
          value={$baseBranch}
          onValueChange={handleBranchChange}
        />

        <span class="text-muted-foreground/40">→</span>

        <!-- Current branch (right) - your working state -->
        <div
          role="status"
          onmouseenter={() => setHover({ label: 'current branch', description: 'your working branch with changes' })}
          onmouseleave={clearHover}
        >
          <Badge variant="outline" class="font-mono text-xs px-2 py-0.5 border-foreground/20">
            {$repoInfo.current_branch}
          </Badge>
        </div>
      </div>
    {/if}
  </div>

  <div class="flex items-center gap-2">
    {#if $repoInfo}
      <Button
        variant="ghost"
        size="sm"
        onclick={refresh}
        disabled={$isLoading}
        class="text-xs h-7 px-2"
        onmouseenter={() => setHover({ label: 'refresh', description: 'reload file changes from disk' })}
        onmouseleave={clearHover}
      >
        {#if $isLoading}
          <span class="animate-pulse">...</span>
        {:else}
          <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
        {/if}
      </Button>
    {/if}

    <Button
      size="sm"
      onclick={handleSelectFolder}
      class="text-xs h-7"
      onmouseenter={() => setHover({
        label: $repoInfo ? 'change repository' : 'open repository',
        description: 'select a git repository folder'
      })}
      onmouseleave={clearHover}
    >
      {$repoInfo ? 'change' : 'open'}
    </Button>
  </div>
</header>

{#if $error}
  <div class="px-4 py-2 bg-destructive/10 border-b border-destructive/20 text-destructive text-xs">
    {$error}
  </div>
{/if}
