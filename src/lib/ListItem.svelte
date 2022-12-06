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
  if (!service && event.detail === 1 && event.webkitForce === 1) {
    await openService(item.id)
  } else {
    await openInfo(item.id)
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
console.log(item)
console.log(shortcut)
</script>

{#if item}
  <button
    class="
        row
        w-full 
        "
    class:text-red-600="{selected}"
    on:click="{handleClick}"
  >
    <div class="flex w-full justify-between">
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
