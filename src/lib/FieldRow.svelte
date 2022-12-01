<script lang="ts">
import { writeText } from '@tauri-apps/api/clipboard'
import type { Field } from 'src/services/searcher'
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
  type Options,
} from '@tauri-apps/api/notification'

export let field: Field
let clicked = false

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

async function notify(options: Options) {
  let permissionGranted = await isPermissionGranted()
  if (!permissionGranted) {
    const permission = await requestPermission()
    permissionGranted = permission === 'granted'
  }
  if (permissionGranted) {
    sendNotification(options)
  }
}
document.onkeydown = function (event: KeyboardEvent) {
  if (field.shortcut.key === event.key && (event.metaKey || event.ctrlKey)) {
    writeText(field.value).then()
    notify({
      title: 'Searcher-RS',
      body: `Copied ${field.name} value.`,
    }).then()
  }
}
</script>

<div class="row my-2">
  <div class="flex w-full text-xl">
    <div class="flex w-40 justify-end">
      <div
        class="flex w-full justify-end rounded-md border-blue-500 bg-blue-500 p-2"
      >
        <span class="inline-block select-none self-center">{field.name}</span>
      </div>
    </div>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div
      on:click="{() => {
        handleValueClick(field.name, field.value)
      }}"
      class="ml-2 flex grow cursor-pointer justify-between rounded-md border border-stone-900 bg-stone-900 p-2 hover:border-blue-700 hover:transition active:border-blue-900"
    >
      <span class="select-none self-center overflow-hidden text-ellipsis"
        >{field.value}</span
      >
      {#if field.shortcut && field.shortcut.key}
        <div class="flex">
          <div
            class="rounded-md border border-stone-600 bg-stone-900 py-0 px-2"
          >
            <span>{field.shortcut.modifier}</span>
          </div>
          <div
            class="ml-1 rounded-md border border-stone-600 bg-stone-900 py-0 px-2"
          >
            <span>{field.shortcut.key}</span>
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>
