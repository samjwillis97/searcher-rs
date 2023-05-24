<script lang="ts">
import { writeText } from '@tauri-apps/api/clipboard'
import type { Field } from '../services/searcher'
import { get_config, type Config } from '../services/config'
import { notify } from '../services/notifications'
import { onDestroy, onMount } from 'svelte'

export let field: Field
let clicked = false

let isActive = false

let config: Config
get_config(true).then((value) => {
  config = value
})

function handleValueClick(field: string, value: string) {
  if (clicked) {
    writeText(value).then()
    notify({
      title: 'Searcher-RS',
      body: `Copied ${field} value.`,
    }).then()
  } else {
    clicked = true
  }

  setTimeout(function () {
    clicked = false
  }, 333)
}

onMount(() => {
  document.addEventListener('keyup', handleKeyUp)
  document.addEventListener('keydown', handleKeyDown)
})

onDestroy(() => {
  document.removeEventListener('keyup', handleKeyUp)
  document.removeEventListener('keydown', handleKeyDown)
})

function handleKeyDown(event: KeyboardEvent) {
  if (
    field.shortcut === event.key &&
    ((config?.app_settings?.modifier_key == 'Cmd' && event.metaKey) ||
      (config?.app_settings?.modifier_key == 'Ctrl' && event.ctrlKey))
  ) {
    isActive = true
  }
}

function handleKeyUp(event: KeyboardEvent) {
  if (
    (config?.app_settings?.modifier_key == 'Cmd' && event.key === 'Meta') ||
    (config?.app_settings?.modifier_key == 'Ctrl' && event.key === 'Ctrl') ||
    event.key === field.shortcut
  ) {
    isActive = false
    writeText(field.value).then()
    notify({
      title: 'Searcher-RS',
      body: `Copied ${field.name} value.`,
    }).then()
  }
}
</script>

<div class="row text-sm font-light text-zinc-200">
  <div class="text-l flex w-full">
    <div
      class="flex w-32 justify-end border-r border-zinc-500 border-opacity-50"
    >
      <div class="flex w-full justify-end pr-3">
        <span class="inline-block select-none self-center">{field.name}</span>
      </div>
    </div>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- TODO: hover:transition, active:border-blue etc. -->
    <div
      on:click="{() => {
        handleValueClick(field.name, field.value)
      }}"
      class="flex grow rounded-md px-1 py-1 "
    >
      <div
        class="flex grow cursor-pointer justify-between rounded px-2 py-1 font-medium hover:bg-zinc-700 hover:bg-opacity-20 active:bg-zinc-600 active:bg-opacity-20"
        class:bg-zinc-600="{isActive}"
        class:bg-opacity-20="{isActive}"
      >
        <span class="select-none self-center overflow-hidden text-ellipsis"
          >{field.value}</span
        >
        {#if field.shortcut}
          <div class="flex select-none">
            <div
              class="rounded-md border border-zinc-500 border-opacity-50 bg-zinc-900 bg-opacity-95 px-2 py-0"
            >
              <span>{config?.app_settings?.modifier_key}</span>
            </div>
            <div
              class="ml-1.5 rounded-md border border-zinc-500  border-opacity-50 bg-zinc-900 bg-opacity-95 px-2"
            >
              <span>{field.shortcut}</span>
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>
