<script lang="ts">
    import type { Field } from "src/services/searcher";

    import { writeText } from '@tauri-apps/api/clipboard';
    import { isPermissionGranted, requestPermission, sendNotification, type Options } from '@tauri-apps/api/notification';

    export let field: Field;
    let clicked = false

    function handleValueClick(field: string, value: string) {
        if (clicked) {
            writeText(value).then();
            notify({
                title: 'Searcher-RS',
                body: `Copied ${field} value.`
            }).then();
        } else {
            clicked = true
        }

        setTimeout(function() {
            clicked = false
        }, 333)
    }

    async function notify(options: Options) {
        let permissionGranted = await isPermissionGranted();
        if (!permissionGranted) {
          const permission = await requestPermission();
          permissionGranted = permission === 'granted';
        }
        if (permissionGranted) {
          sendNotification(options);
        }
    }

</script>

<div class="row my-2">
    <div class="w-full flex text-xl">
        <div class="flex w-40 justify-end">
            <div class="flex w-full justify-end rounded-md border-blue-500 bg-blue-500 p-2">
                <span class="self-center select-none inline-block">{field.name}</span>
            </div>
        </div>
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div on:click={() => {handleValueClick(field.name, field.value)}} class="ml-2 flex justify-between grow border rounded-md border-stone-900 bg-stone-900 p-2 cursor-pointer hover:border-blue-700 active:border-blue-900 hover:transition">
            <span class="self-center select-none text-ellipsis overflow-hidden">{field.value}</span>
            {#if field.shortcut}
                <span>{field.shortcut.key}</span>
            {/if}
        </div>
    </div>
</div>
