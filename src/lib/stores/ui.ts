import { writable } from 'svelte/store';

export interface HoverContext {
  label: string;
  description?: string;
  shortcut?: string;
}

export const hoverContext = writable<HoverContext | null>(null);

export function setHover(context: HoverContext | null) {
  hoverContext.set(context);
}

export function clearHover() {
  hoverContext.set(null);
}
