<script lang="ts">
  import { showHelp } from '../stores';

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      showHelp.set(false);
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      showHelp.set(false);
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

{#if $showHelp}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
    on:click={handleBackdropClick}
    role="dialog"
    aria-modal="true"
    aria-labelledby="help-title"
  >
    <div class="bg-card border border-border rounded-lg shadow-xl max-w-md w-full mx-4 overflow-hidden">
      <div class="px-5 py-4 border-b border-border">
        <h2 id="help-title" class="text-sm font-bold uppercase tracking-wide">Keyboard Shortcuts</h2>
      </div>

      <div class="px-5 py-4 space-y-5 max-h-[60vh] overflow-y-auto">
        <section>
          <h3 class="text-[10px] uppercase tracking-widest text-muted-foreground mb-2">Navigation</h3>
          <div class="space-y-1.5">
            <div class="flex justify-between text-xs">
              <span class="text-muted-foreground">Move down</span>
              <span class="flex gap-1">
                <kbd class="px-1.5 py-0.5 bg-muted rounded text-[10px]">j</kbd>
                <kbd class="px-1.5 py-0.5 bg-muted rounded text-[10px]">↓</kbd>
              </span>
            </div>
            <div class="flex justify-between text-xs">
              <span class="text-muted-foreground">Move up</span>
              <span class="flex gap-1">
                <kbd class="px-1.5 py-0.5 bg-muted rounded text-[10px]">k</kbd>
                <kbd class="px-1.5 py-0.5 bg-muted rounded text-[10px]">↑</kbd>
              </span>
            </div>
            <div class="flex justify-between text-xs">
              <span class="text-muted-foreground">Select first file</span>
              <kbd class="px-1.5 py-0.5 bg-muted rounded text-[10px]">Enter</kbd>
            </div>
            <div class="flex justify-between text-xs">
              <span class="text-muted-foreground">Clear selection</span>
              <kbd class="px-1.5 py-0.5 bg-muted rounded text-[10px]">Esc</kbd>
            </div>
            <div class="flex justify-between text-xs">
              <span class="text-muted-foreground">Focus search</span>
              <kbd class="px-1.5 py-0.5 bg-muted rounded text-[10px]">/</kbd>
            </div>
          </div>
        </section>

        <section>
          <h3 class="text-[10px] uppercase tracking-widest text-muted-foreground mb-2">Actions</h3>
          <div class="space-y-1.5">
            <div class="flex justify-between text-xs">
              <span class="text-muted-foreground">Open repository</span>
              <kbd class="px-1.5 py-0.5 bg-muted rounded text-[10px]">o</kbd>
            </div>
            <div class="flex justify-between text-xs">
              <span class="text-muted-foreground">Refresh</span>
              <kbd class="px-1.5 py-0.5 bg-muted rounded text-[10px]">r</kbd>
            </div>
            <div class="flex justify-between text-xs">
              <span class="text-muted-foreground">Toggle view mode</span>
              <kbd class="px-1.5 py-0.5 bg-muted rounded text-[10px]">v</kbd>
            </div>
          </div>
        </section>

        <section>
          <h3 class="text-[10px] uppercase tracking-widest text-muted-foreground mb-2">Filters</h3>
          <div class="space-y-1.5">
            <div class="flex justify-between text-xs">
              <span class="text-muted-foreground">Toggle added files</span>
              <kbd class="px-1.5 py-0.5 bg-muted rounded text-[10px]">1</kbd>
            </div>
            <div class="flex justify-between text-xs">
              <span class="text-muted-foreground">Toggle modified files</span>
              <kbd class="px-1.5 py-0.5 bg-muted rounded text-[10px]">2</kbd>
            </div>
            <div class="flex justify-between text-xs">
              <span class="text-muted-foreground">Toggle deleted files</span>
              <kbd class="px-1.5 py-0.5 bg-muted rounded text-[10px]">3</kbd>
            </div>
            <div class="flex justify-between text-xs">
              <span class="text-muted-foreground">Toggle cosmetic changes</span>
              <kbd class="px-1.5 py-0.5 bg-muted rounded text-[10px]">4</kbd>
            </div>
          </div>
        </section>

        <section>
          <h3 class="text-[10px] uppercase tracking-widest text-muted-foreground mb-2">Tree</h3>
          <div class="space-y-1.5">
            <div class="flex justify-between text-xs">
              <span class="text-muted-foreground">Collapse all</span>
              <kbd class="px-1.5 py-0.5 bg-muted rounded text-[10px]">[</kbd>
            </div>
            <div class="flex justify-between text-xs">
              <span class="text-muted-foreground">Expand all</span>
              <kbd class="px-1.5 py-0.5 bg-muted rounded text-[10px]">]</kbd>
            </div>
          </div>
        </section>
      </div>

      <div class="px-5 py-3 border-t border-border bg-muted/30">
        <p class="text-[10px] text-muted-foreground/60 text-center">
          Press <kbd class="px-1 py-0.5 bg-muted rounded text-[9px]">?</kbd> to toggle this help
        </p>
      </div>
    </div>
  </div>
{/if}
