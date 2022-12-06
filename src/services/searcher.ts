import { invoke } from '@tauri-apps/api/tauri'

export type SearchResult = {
  id: string
  value: string
  indices: number[]
  score: number
}

export type Field = {
  name: string
  value: string
  shortcut: string
}

export async function search(
  id: string,
  term: string
): Promise<SearchResult[]> {
  console.log('SEARCH')
  return invoke('search', { id: id, term: term })
}
