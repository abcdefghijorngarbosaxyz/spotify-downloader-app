import type { ComponentType } from 'svelte';

export type Option<T> = T | null;

export interface Page {
  [key: string]: { name: string; icon: ComponentType };
}
