<style>
</style>

<script lang="ts">
import type { Field } from 'src/services/searcher'

import { closeWindow, getInfo } from '../../../services/commands'
import type { InfoData } from './+page'
import FieldRow from '$lib/FieldRow.svelte'

export let data: InfoData

let fields: Field[] = []

getInfo(data.id).then((v) => {
  fields = v
  console.log(v)
})

document.onkeydown = function (event: KeyboardEvent) {
  if (event.key === 'Escape') {
    closeWindow().then()
  }
}
</script>

<div class="col-auto">
  <span>Info Screen</span>
  <span>{data.id}</span>

  {#each fields as field}
    {#if field.value}
      <FieldRow field="{field}" />
    {/if}
  {/each}
</div>
