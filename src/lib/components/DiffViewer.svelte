<script lang="ts">
  import { currentDiff, selectedFile, viewMode, repoPath } from '../stores/diff';
  import { openInEditor } from '../tauri';
  import { Button } from '$lib/components/ui/button';
  import * as ToggleGroup from '$lib/components/ui/toggle-group';
  import { ScrollArea } from '$lib/components/ui/scroll-area';
  import { cn } from '$lib/utils';

  async function handleOpenInEditor() {
    if ($repoPath && $selectedFile) {
      await openInEditor($repoPath, $selectedFile);
    }
  }

  function getLineClass(lineType: string): string {
    switch (lineType) {
      case 'add': return 'bg-green-500/5 text-green-300';
      case 'delete': return 'bg-red-500/5 text-red-300';
      default: return 'text-foreground/70';
    }
  }

  function getGutterClass(lineType: string): string {
    switch (lineType) {
      case 'add': return 'bg-green-500/10 text-green-500/70';
      case 'delete': return 'bg-red-500/10 text-red-500/70';
      default: return 'bg-muted/50 text-muted-foreground/50';
    }
  }

  function getLinePrefix(lineType: string): string {
    switch (lineType) {
      case 'add': return '+';
      case 'delete': return '-';
      default: return ' ';
    }
  }
</script>

<main class="flex-1 flex flex-col overflow-hidden bg-background">
  {#if !$selectedFile}
    <div class="flex-1 flex items-center justify-center text-muted-foreground">
      <div class="text-center">
        <div class="text-2xl mb-2 text-muted-foreground/30">[  ]</div>
        <p class="text-xs">select file</p>
      </div>
    </div>
  {:else if $currentDiff}
    <div class="flex items-center justify-between px-3 py-1.5 bg-card border-b border-border">
      <div class="flex items-center gap-2">
        <span class="text-xs text-foreground/70">{$currentDiff.path}</span>
      </div>
      <div class="flex items-center gap-2">
        <ToggleGroup.Root
          type="single"
          value={$viewMode}
          onValueChange={(v) => { if (v) $viewMode = v as 'unified' | 'split'; }}
          class="bg-muted/50 rounded p-0.5"
        >
          <ToggleGroup.Item value="unified" class="text-[10px] px-2 py-0.5 h-5 data-[state=on]:bg-background rounded-sm">
            unified
          </ToggleGroup.Item>
          <ToggleGroup.Item value="split" class="text-[10px] px-2 py-0.5 h-5 data-[state=on]:bg-background rounded-sm">
            split
          </ToggleGroup.Item>
        </ToggleGroup.Root>
        <Button variant="ghost" size="sm" onclick={handleOpenInEditor} class="text-[10px] h-5 px-2">
          open
        </Button>
      </div>
    </div>

    <ScrollArea class="flex-1">
      {#if $currentDiff.is_binary}
        <div class="flex items-center justify-center h-full text-muted-foreground py-20 text-xs">
          binary
        </div>
      {:else if $currentDiff.hunks.length === 0}
        <div class="flex items-center justify-center h-full text-muted-foreground py-20 text-xs">
          no changes
        </div>
      {:else if $viewMode === 'unified'}
        <div class="text-xs">
          {#each $currentDiff.hunks as hunk}
            <div class="bg-blue-500/5 text-blue-400/70 px-3 py-0.5 text-[10px] border-y border-border">
              @@ -{hunk.old_start},{hunk.old_lines} +{hunk.new_start},{hunk.new_lines} @@
            </div>
            {#each hunk.lines as line}
              <div class="flex {getLineClass(line.line_type)}">
                <span class="w-10 text-right px-1.5 select-none {getGutterClass(line.line_type)} border-r border-border/50 text-[10px] leading-5 tabular-nums">
                  {line.old_lineno ?? ''}
                </span>
                <span class="w-10 text-right px-1.5 select-none {getGutterClass(line.line_type)} border-r border-border/50 text-[10px] leading-5 tabular-nums">
                  {line.new_lineno ?? ''}
                </span>
                <span class="w-4 text-center select-none {getGutterClass(line.line_type)} text-[10px] leading-5">
                  {getLinePrefix(line.line_type)}
                </span>
                <pre class="flex-1 px-2 whitespace-pre-wrap break-all leading-5">{line.content}</pre>
              </div>
            {/each}
          {/each}
        </div>
      {:else}
        <div class="text-xs flex min-w-full">
          <div class="flex-1 border-r border-border">
            {#each $currentDiff.hunks as hunk}
              <div class="bg-blue-500/5 text-blue-400/70 px-3 py-0.5 text-[10px] border-y border-border">
                @@ -{hunk.old_start},{hunk.old_lines} @@
              </div>
              {#each hunk.lines as line}
                {#if line.line_type !== 'add'}
                  <div class={cn("flex", line.line_type === 'delete' ? 'bg-red-500/5 text-red-300' : 'text-foreground/70')}>
                    <span class={cn("w-10 text-right px-1.5 select-none border-r border-border/50 text-[10px] leading-5 tabular-nums", line.line_type === 'delete' ? 'bg-red-500/10 text-red-500/70' : 'bg-muted/50 text-muted-foreground/50')}>
                      {line.old_lineno ?? ''}
                    </span>
                    <pre class="flex-1 px-2 whitespace-pre-wrap break-all leading-5">{line.content}</pre>
                  </div>
                {:else}
                  <div class="flex text-muted-foreground/10">
                    <span class="w-10 text-right px-1.5 select-none bg-muted/30 border-r border-border/50 leading-5"></span>
                    <pre class="flex-1 px-2 whitespace-pre-wrap break-all leading-5"></pre>
                  </div>
                {/if}
              {/each}
            {/each}
          </div>
          <div class="flex-1">
            {#each $currentDiff.hunks as hunk}
              <div class="bg-blue-500/5 text-blue-400/70 px-3 py-0.5 text-[10px] border-y border-border">
                @@ +{hunk.new_start},{hunk.new_lines} @@
              </div>
              {#each hunk.lines as line}
                {#if line.line_type !== 'delete'}
                  <div class={cn("flex", line.line_type === 'add' ? 'bg-green-500/5 text-green-300' : 'text-foreground/70')}>
                    <span class={cn("w-10 text-right px-1.5 select-none border-r border-border/50 text-[10px] leading-5 tabular-nums", line.line_type === 'add' ? 'bg-green-500/10 text-green-500/70' : 'bg-muted/50 text-muted-foreground/50')}>
                      {line.new_lineno ?? ''}
                    </span>
                    <pre class="flex-1 px-2 whitespace-pre-wrap break-all leading-5">{line.content}</pre>
                  </div>
                {:else}
                  <div class="flex text-muted-foreground/10">
                    <span class="w-10 text-right px-1.5 select-none bg-muted/30 border-r border-border/50 leading-5"></span>
                    <pre class="flex-1 px-2 whitespace-pre-wrap break-all leading-5"></pre>
                  </div>
                {/if}
              {/each}
            {/each}
          </div>
        </div>
      {/if}
    </ScrollArea>
  {:else}
    <div class="flex-1 flex items-center justify-center text-muted-foreground text-xs">
      ...
    </div>
  {/if}
</main>
