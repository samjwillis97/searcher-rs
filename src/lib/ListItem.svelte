<script lang="ts">
import { get_config, type Config } from '../services/config'

import { openInfo, openService } from '../services/commands'

import type { SearchResult } from '../services/searcher'

export let service: string = ''
export let item: SearchResult
export let shortcut: string | undefined
export let selected: boolean = false

$: htmlString = generateHtmlString(item)

let config: Config
get_config(false).then((v) => {
  config = v
})

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
        "
    class:text-indigo-500="{selected}"
    class:bg-zinc-700="{selected}"
    class:bg-opacity-30="{selected}"
    class:text-zinc-200="{!selected}"
    on:click="{handleClick}"
  >
    <div class="flex w-full flex-row items-center justify-between py-0.5">
      <div>
        {@html htmlString}
      </div>

      <div class="flex">
        {#if shortcut}
          <div
            class="rounded-md border border-stone-600 bg-stone-900 py-0 px-2"
          >
            <p>{config?.app_settings?.modifier_key}</p>
          </div>
          <div
            class="ml-1.5 rounded-md border border-stone-600 bg-stone-900 py-0 px-2"
          >
            <p>{shortcut}</p>
          </div>
        {/if}
      </div>
    </div>
  </button>
{/if}
