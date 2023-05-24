<script lang="ts">
export const prerender = 'auto'
import type { Field } from 'src/services/searcher'

import { get_config, type Config } from '../../../services/config'
import {
  getInfo,
  resizeInfoWindow,
  openPreviousService,
  closeWindow,
} from '../../../services/commands'
import type { InfoData } from './+page'
import FieldRow from '$lib/FieldRow.svelte'
import { onDestroy, onMount } from 'svelte'

export let data: InfoData

let fields: Field[] = []

let config: Config
get_config(true).then((value) => {
  config = value
})

getInfo(data.id).then((v) => {
  fields = v
  resizeInfoWindow((v.length + 1) * 36 - 4).then()
})

function handleKeyUp(event: KeyboardEvent) {
  if (event.key === 'Escape') {
    if (config.app_settings.escape_closes_info) {
      closeWindow().then()
    } else {
      openPreviousService().then()
    }
  }
}

onMount(() => {
  document.addEventListener('keyup', handleKeyUp)
})

onDestroy(() => {
  document.removeEventListener('keyup', handleKeyUp)
})
</script>

<main style="height: 100%;" class="bg-zinc-900 bg-opacity-95">
  <div class="col-auto">
    {#each fields as field}
      {#if field.value}
        <div class="border-b border-zinc-500 border-opacity-50 last:border-0">
          <FieldRow field="{field}" />
        </div>
      {/if}
    {/each}
  </div>
</main>
