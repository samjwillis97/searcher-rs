<script lang="ts">
import { get_config, type Config } from '../services/config'

import { openInfo, openService } from '../services/commands'

import type { SearchResult } from '../services/searcher'
import { onDestroy, onMount } from 'svelte'

export let service: string = ''
export let item: SearchResult
export let shortcut: string | undefined
export let selected: boolean = false

let isActive = false

$: htmlString = generateHtmlString(item)

let config: Config
get_config(false).then((v) => {
  config = v
})

onMount(() => {
  document.addEventListener('keyup', handleKeyUp)
  document.addEventListener('keydown', handleKeyDown)
})

onDestroy(() => {
  document.removeEventListener('keyup', handleKeyUp)
  document.removeEventListener('keydown', handleKeyDown)
})

function handleKeyUp(event: KeyboardEvent) {
  if (service == '') {
    if (
      (config?.app_settings?.modifier_key == 'Cmd' && event.metaKey) ||
      (config?.app_settings?.modifier_key == 'Ctrl' && event.ctrlKey) ||
      (shortcut && event.key === shortcut)
    ) {
      isActive = false
    }
  }
}

function handleKeyDown(event: KeyboardEvent) {
  if (service == '') {
    if (
      ((config?.app_settings?.modifier_key == 'Cmd' && event.metaKey) ||
        (config?.app_settings?.modifier_key == 'Ctrl' && event.ctrlKey)) &&
      shortcut &&
      event.key === shortcut
    ) {
      isActive = true
    }
  }
}

async function handleClick(event: MouseEvent) {
  //@ts-ignore
  if (event.detail === 1 && event.webkitForce === 1) {
    if (!service) {
      await openService(item.id)
    } else {
      await openInfo(item.id)
    }
  }
}

// TODO: Handle keyup and keydown for nice lil highlight

function generateHtmlString(item: SearchResult): String {
  if (!item) return ''
  return item.value
    .split('')
    .map((val, i) => {
      if (val === ' ') {
        val = '&nbsp;'
      }
      if (item.indices.includes(i)) {
        return `<b>${val}</b>`
      } else {
        return val
      }
    })
    .join('')
}
</script>

{#if item}
  <button
    class="
        row
        w-full 
        cursor-pointer
        rounded-md
        py-0.5
        px-2
        text-sm
        font-medium
        outline-none
        hover:bg-zinc-700
        hover:bg-opacity-20
        active:bg-zinc-600
        active:bg-opacity-20
        "
    class:text-indigo-500="{selected}"
    class:bg-zinc-700="{selected && !isActive}"
    class:bg-opacity-20="{selected || isActive}"
    class:text-zinc-200="{!selected}"
    class:bg-zinc-600="{isActive}"
    on:click="{handleClick}"
  >
    <div class="flex w-full flex-row items-center justify-between py-0.5">
      <div>
        {@html htmlString}
      </div>

      <div class="flex select-none">
        {#if shortcut}
          <div
            class="rounded-md border border-zinc-500 border-opacity-50 bg-zinc-900 bg-opacity-95 py-0 px-2"
          >
            <p>{config?.app_settings?.modifier_key}</p>
          </div>
          <div
            class="ml-1.5 rounded-md border border-zinc-500 border-opacity-50 bg-zinc-900 bg-opacity-95 py-0 px-2"
          >
            <p>{shortcut}</p>
          </div>
        {/if}
      </div>
    </div>
  </button>
{/if}
