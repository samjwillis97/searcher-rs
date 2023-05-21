<script lang="ts">
import { listen } from '@tauri-apps/api/event'
import { createEventDispatcher } from 'svelte'
import { search } from '../services/searcher'

export let isFocused = true
export let service = ''
export let hasSearchValues = false

let placeholder = 'Search...'
let searchValue = ''

let searchBar: HTMLInputElement
const dispatch = createEventDispatcher()

async function onInput() {
  search(service, searchValue).then((v) => {
    dispatch('newValues', {
      values: v,
    })
  })
}

listen('ClearSearch', (_) => {
  searchValue = ''
})

listen('FocusSearch', (_) => {
  isFocused = true
  if (searchBar) {
    searchBar.focus()
    onInput().then()
  }
  if (service !== '') {
    console.log('Searching ' + service)
  }
})
</script>

<main class="row">
  <div class="flex w-full flex-row">
    {#if service !== ''}
      <div
        class="flex rounded-tl-lg border-b border-zinc-500
      border-opacity-50
      bg-indigo-900
      bg-opacity-95
        "
        class:rounded-bl-lg="{!hasSearchValues}"
      >
        <span class="text-l px-2 pt-2.5 text-zinc-200">{service}</span>
      </div>
    {/if}
    <input
      class="
      grow
      border-b
      border-zinc-500
      border-opacity-50
      bg-zinc-900
      bg-opacity-95
      py-2.5
      px-5
      font-medium
      text-zinc-200
      outline-none
      "
      class:rounded-t-lg="{service === ''}"
      class:rounded-tr-lg="{service !== ''}"
      class:rounded-br-lg="{!hasSearchValues}"
      class:rounded-bl-lg="{service === '' && !hasSearchValues}"
      id="search"
      autocomplete="off"
      autocorrect="off"
      autocapitalize="off"
      spellcheck="false"
      selected="{isFocused}"
      placeholder="{placeholder}"
      bind:this="{searchBar}"
      bind:value="{searchValue}"
      on:input="{onInput}"
    />
  </div>
</main>
