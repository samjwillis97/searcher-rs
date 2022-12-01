import { invoke } from '@tauri-apps/api/tauri'

export type SearchResult = {
  id: string
  value: string
  indices: number[]
  score: number
}

export type Shortcut = {
  modifier: ModifierKey
  key: string
}

export enum ModifierKey {
  Cmd = 'cmd',
  Ctrl = 'ctrl',
}

export type Field = {
  name: string
  value: string
  shortcut: Shortcut
}

export async function search(
  id: string,
  term: string
): Promise<SearchResult[]> {
  return invoke('search', { id: id, term: term })
}
