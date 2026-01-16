<script lang="ts">
  import { repoPath, repoInfo, baseBranch, isLoading, changedFiles, selectedFile, currentDiff, error } from '../stores/diff';
  import { selectFolder, getRepoInfo, getChangedFiles } from '../tauri';
  import { Button } from '$lib/components/ui/button';
  import * as Select from '$lib/components/ui/select';
  import { Badge } from '$lib/components/ui/badge';

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
      await refresh();
    } catch (e) {
      $error = e as string;
    } finally {
      $isLoading = false;
    }
  }

  async function refresh() {
    if (!$repoPath || !$baseBranch) return;

    $isLoading = true;
    $error = null;

    try {
      const files = await getChangedFiles($repoPath, $baseBranch);
      $changedFiles = files;
      $selectedFile = null;
      $currentDiff = null;
    } catch (e) {
      $error = e as string;
    } finally {
      $isLoading = false;
    }
  }

  $: localBranches = $repoInfo?.branches.filter(b => !b.is_remote) ?? [];
  $: selectedBranchLabel = localBranches.find(b => b.name === $baseBranch)?.name ?? $baseBranch;
</script>

<header class="flex items-center justify-between px-4 py-2.5 bg-card border-b border-border">
  <div class="flex items-center gap-4">
    <h1 class="text-sm font-bold tracking-tight text-foreground uppercase">Difference</h1>

    {#if $repoInfo}
      <div class="flex items-center gap-2 text-xs">
        <Badge variant="outline" class="font-mono text-xs px-2 py-0.5 border-foreground/20">
          {$repoInfo.current_branch}
        </Badge>
        <span class="text-muted-foreground">/</span>
        <Select.Root
          type="single"
          value={{ value: $baseBranch, label: selectedBranchLabel }}
          onValueChange={(v) => { if (v) { $baseBranch = v.value; refresh(); } }}
        >
          <Select.Trigger class="w-[120px] h-7 text-xs">
            {selectedBranchLabel}
          </Select.Trigger>
          <Select.Content>
            {#each localBranches as branch}
              <Select.Item value={branch.name} label={branch.name} class="text-xs">
                {branch.name}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
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

    <Button size="sm" onclick={handleSelectFolder} class="text-xs h-7">
      {$repoInfo ? 'change' : 'open'}
    </Button>
  </div>
</header>

{#if $error}
  <div class="px-4 py-2 bg-destructive/10 border-b border-destructive/20 text-destructive text-xs">
    {$error}
  </div>
{/if}
