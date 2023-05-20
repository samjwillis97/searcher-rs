<script lang="ts">
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
  resizeInfoWindow((v.length + 1) * 45.25 + 10).then()
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

<div class="col-auto mx-3">
  {#each fields as field}
    {#if field.value}
      <FieldRow field="{field}" />
    {/if}
  {/each}
</div>
