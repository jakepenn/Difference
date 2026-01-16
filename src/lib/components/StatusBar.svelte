<script lang="ts">
  import { hoverContext, repoPath, repoInfo } from '../stores';
</script>

<footer class="h-6 flex items-center justify-between px-3 bg-card border-t border-border text-[10px] text-muted-foreground">
  <div class="flex items-center gap-3">
    {#if $hoverContext}
      <span class="text-foreground">{$hoverContext.label}</span>
      {#if $hoverContext.description}
        <span class="text-muted-foreground/60">{$hoverContext.description}</span>
      {/if}
    {:else if $repoInfo}
      <span class="text-muted-foreground/60">{$repoPath}</span>
    {:else}
      <span class="text-muted-foreground/40">ready</span>
    {/if}
  </div>

  <div class="flex items-center gap-3">
    {#if $hoverContext?.shortcut}
      <span class="text-muted-foreground/40 font-medium">{$hoverContext.shortcut}</span>
    {/if}
  </div>
</footer>
