<script lang="ts">
  import { tick } from 'svelte';
  import * as Command from '$lib/components/ui/command';
  import * as Popover from '$lib/components/ui/popover';
  import { Button } from '$lib/components/ui/button';
  import { cn } from '$lib/utils';
  import { setHover, clearHover } from '../stores';
  import type { BranchInfo } from '../types';

  interface Props {
    branches: BranchInfo[];
    value: string;
    onValueChange: (value: string) => void;
  }

  let { branches, value, onValueChange }: Props = $props();

  let open = $state(false);
  let triggerRef = $state<HTMLButtonElement>(null!);

  const selectedLabel = $derived(
    branches.find((b) => b.name === value)?.name ?? value
  );

  function closeAndFocusTrigger() {
    open = false;
    tick().then(() => triggerRef?.focus());
  }

  function handleSelect(branchName: string) {
    onValueChange(branchName);
    closeAndFocusTrigger();
  }
</script>

<div
  role="group"
  onmouseenter={() => setHover({ label: 'base branch', description: 'comparing changes against this branch' })}
  onmouseleave={clearHover}
>
  <Popover.Root bind:open>
    <Popover.Trigger bind:ref={triggerRef}>
      {#snippet child({ props })}
        <Button
          {...props}
          variant="outline"
          class="w-[140px] h-7 justify-between text-xs px-2 font-mono"
          role="combobox"
          aria-expanded={open}
        >
          <span class="truncate">{selectedLabel}</span>
          <span class="text-[10px] text-muted-foreground ml-1">{open ? '▴' : '▾'}</span>
        </Button>
      {/snippet}
    </Popover.Trigger>
    <Popover.Content class="w-[200px] p-0" align="start">
      <Command.Root>
        <Command.Input placeholder="search branches..." class="h-8 text-xs" />
        <Command.List class="max-h-[200px]">
          <Command.Empty class="py-4 text-xs text-center text-muted-foreground">
            no branches found
          </Command.Empty>
          <Command.Group>
            {#each branches as branch (branch.name)}
              <Command.Item
                value={branch.name}
                onSelect={() => handleSelect(branch.name)}
                class="text-xs"
              >
                <span class={cn(
                  "w-3 text-center mr-1",
                  value === branch.name ? "text-foreground" : "text-transparent"
                )}>
                  ✓
                </span>
                <span class="font-mono truncate">{branch.name}</span>
                {#if branch.is_current}
                  <span class="ml-auto text-[10px] text-muted-foreground">current</span>
                {/if}
              </Command.Item>
            {/each}
          </Command.Group>
        </Command.List>
      </Command.Root>
    </Popover.Content>
  </Popover.Root>
</div>
