<script lang="ts">
import { get_config, type Config } from '../services/config'

import { listen } from '@tauri-apps/api/event'
import { resizeWindow } from '../services/commands'
import type { SearchResult } from '../services/searcher'
import ListItem from './ListItem.svelte'

export let service: string = ''
export let items: SearchResult[] = []
export let shortcuts: string[] = []
export let currentSelection: number = 0

let maxToShow = 5

let config: Config
get_config(false).then((v) => {
  config = v
  maxToShow = config.user_settings.entries_to_show
})

let viewport_top = 0
let viewport_bottom = config ? config.user_settings.entries_to_show : 5
let viewport_items: SearchResult[] = []

listen('ClearSearch', (_) => {
  viewport_top = 0
  viewport_bottom = config ? config.user_settings.entries_to_show : 5
})

$: {
  let toShow = items.length

  if (toShow > maxToShow) {
    toShow = maxToShow
  }

  resizeWindow((toShow + 1) * 46.25).then()
}

$: {
  if (currentSelection > viewport_bottom - 1) {
    viewport_top = currentSelection - maxToShow + 1
    viewport_bottom = currentSelection + 1
  }
  if (currentSelection < viewport_top) {
    viewport_top = currentSelection
    viewport_bottom = currentSelection + maxToShow
  }

  viewport_items = items.slice(viewport_top, viewport_bottom)
}
</script>

<div class="col overflow-hidden rounded-b-lg bg-zinc-900 bg-opacity-95">
  {#if viewport_items && viewport_items.length > 0}
    {#each viewport_items as item, index}
      <div class="p-1">
        {#if shortcuts && shortcuts.length > index - 1}
          <ListItem
            service="{service}"
            item="{item}"
            selected="{index === currentSelection - viewport_top}"
            shortcut="{shortcuts[index]}"
          />
        {:else}
          <ListItem
            service="{service}"
            item="{item}"
            selected="{index === currentSelection - viewport_top}"
            shortcut="{undefined}"
          />
        {/if}
      </div>
    {/each}
  {/if}
</div>
