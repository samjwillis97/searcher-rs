<style>
</style>

<script lang="ts">
import List from '$lib/List.svelte'
import Search from '$lib/Search.svelte'
import {
  get_config,
  type Config,
  type SearchServiceConfig,
} from '../services/config'
import type { SearchResult } from '../services/searcher'
import { listen } from '@tauri-apps/api/event'
import { appWindow } from '@tauri-apps/api/window'
import { closeSearch, openInfo, openService } from '../services/commands'

let service: string = ''
let isFocused: boolean = true
let currentSelection: number = 0

let config: Config
let searchServicesShortcutMap: Map<string, SearchServiceConfig> = new Map()
get_config(true).then((value) => {
  if (service === '') {
    value.search_services.forEach((value) => {
      searchServicesShortcutMap.set(value.shortcut, value)
    })
  }
  config = value
})

let items: SearchResult[] = []
let shortcuts: string[] = []
function handleNewValues(event: CustomEvent) {
  items = event.detail.values
  if (service == '') {
    shortcuts = []
    const services = [...searchServicesShortcutMap.values()]
    for (let i = 0; i < items.length; i++) {
      const service = services.find((value) => value.name === items[i].value)
      if (service) {
        shortcuts.push(service.shortcut)
      } else {
        shortcuts.push(undefined as any)
      }
    }
  } else {
    shortcuts = []
  }
}

listen('ClearSearch', (_) => {
  items = []
  currentSelection = 0
})

listen('SetService', (value) => {
  service = value.payload?.toString()
})

document.onkeydown = function (event: KeyboardEvent) {
  switch (event.key) {
    case 'Escape':
      // TODO: Go back instead of closing depending on the state
      closeSearch().then()
      break
    case 'Enter':
      if (!service) {
        openService(items[currentSelection].id).then()
      } else {
        openInfo(items[currentSelection].id).then()
      }
      break
    case 'Tab':
      currentSelection++
      if (currentSelection > items.length - 1) {
        currentSelection = 0
      }
      event.preventDefault()
      break
    case 'n':
      if (event.ctrlKey) {
        currentSelection++
        if (currentSelection > items.length - 1) {
          currentSelection = 0
        }
      }
      break
    case 'p':
      if (event.ctrlKey) {
        currentSelection--
        if (currentSelection < 0) {
          currentSelection = items.length - 1
        }
      }
      break
  }
  if (service == '') {
    if (
      (config?.app_settings?.modifier_key == 'Cmd' && event.metaKey) ||
      (config?.app_settings?.modifier_key == 'Ctrl' && event.ctrlKey)
    ) {
      if (searchServicesShortcutMap.has(event.key)) {
        openService(searchServicesShortcutMap.get(event.key).name).then()
      }
    }
  }
}

appWindow.onFocusChanged(({ payload: focused }) => {
  if (!focused) {
    // closeSearch().then();
  }
})
</script>

<main>
  <Search
    service="{service}"
    isFocused="{isFocused}"
    on:newValues="{handleNewValues}"
  />
  <List
    items="{items}"
    service="{service}"
    shortcuts="{shortcuts}"
    currentSelection="{currentSelection}"
  />
</main>
