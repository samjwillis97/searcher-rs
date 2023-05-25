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
import { onDestroy, onMount } from 'svelte'

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
      const service = services.find((value) => value.name === items[i].value[0])
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
  service = value.payload?.toString() ?? ''
})

function selectNext() {
  selectIndex(currentSelection + 1)
}

function selectPrevious() {
  selectIndex(currentSelection - 1)
}

function selectIndex(index: number) {
  if (index < 0) {
    currentSelection = items.length - 1
  } else if (index > items.length - 1) {
    currentSelection = 0
  } else {
    currentSelection = index
  }
}

function handleKeyUp(event: KeyboardEvent) {
  switch (event.key) {
    case 'Escape':
      if (service !== '' && !config.app_settings.escape_closes_service_search) {
        openService('').then()
      } else {
        closeSearch().then()
      }
      break
    case 'Enter':
      if (!service && items[currentSelection]) {
        openService(items[currentSelection].id).then()
      } else {
        openInfo(items[currentSelection].id).then()
      }
      event.preventDefault()
      break
    case 'Tab':
      event.preventDefault()
      break
  }
  if (service == '') {
    if (
      (config?.app_settings?.modifier_key == 'Cmd' && event.metaKey) ||
      (config?.app_settings?.modifier_key == 'Ctrl' && event.ctrlKey)
    ) {
      if (searchServicesShortcutMap.has(event.key)) {
        const serviceName = searchServicesShortcutMap.get(event.key)?.name
        if (serviceName) {
          openService(serviceName).then()
        }
      }
    }
  }
}

function handleKeyDown(event: KeyboardEvent) {
  switch (event.key) {
    case 'Tab':
      selectNext()
      event.preventDefault()
      break
    case 'n':
      if (event.ctrlKey) selectNext()
      break
    case 'p':
      if (event.ctrlKey) selectPrevious()
      break
  }
}

onMount(() => {
  document.addEventListener('keyup', handleKeyUp)
  document.addEventListener('keydown', handleKeyDown)
})

onDestroy(() => {
  document.removeEventListener('keyup', handleKeyUp)
  document.removeEventListener('keydown', handleKeyDown)
})

appWindow.onFocusChanged(({ payload: focused }) => {
  if (!focused) {
    // TODO: Address this pls
    // closeSearch().then();
  }
})
</script>

<main>
  <!-- TODO: Look at `backdrop-blur-sm` -->
  <div
    class="rounded-md border-x border-t border-zinc-500 border-opacity-50"
    class:border-b="{items && items.length > 0}"
  >
    <Search
      service="{service}"
      isFocused="{isFocused}"
      hasSearchValues="{items && items.length > 0}"
      on:newValues="{handleNewValues}"
    />
    <List
      items="{items}"
      service="{service}"
      shortcuts="{shortcuts}"
      currentSelection="{currentSelection}"
    />
  </div>
</main>
