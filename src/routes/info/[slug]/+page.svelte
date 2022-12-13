<style>
</style>

<script lang="ts">
import type { Field } from 'src/services/searcher'

import {
  closeWindow,
  getInfo,
  resizeInfoWindow,
} from '../../../services/commands'
import type { InfoData } from './+page'
import FieldRow from '$lib/FieldRow.svelte'

export let data: InfoData

let fields: Field[] = []

getInfo(data.id).then((v) => {
  fields = v
  resizeInfoWindow((v.length + 1) * 45.25 + 10).then()
})

document.onkeydown = function (event: KeyboardEvent) {
  if (event.key === 'Escape') {
    closeWindow().then()
  }
}
</script>

<div class="col-auto mx-3">
  {#each fields as field}
    {#if field.value}
      <FieldRow field="{field}" />
    {/if}
  {/each}
</div>
