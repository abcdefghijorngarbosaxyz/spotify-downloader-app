<script lang="ts">
  import Nav from './Nav.svelte';

  import { useTauriEvent } from '@app/lib/utils/useTauriEvent';
  import { TauriEvent } from '@tauri-apps/api/event';
  import { onDestroy, onMount } from 'svelte';
  import { open as dialogOpen } from '@tauri-apps/api/dialog';
  import { open as shellOpen } from '@tauri-apps/api/shell';
  import { DOWNLOAD_FOLDER } from '@app/store';

  let cleanUpSelectDownloadFolder: () => void;

  onMount(() => {
    cleanUpSelectDownloadFolder = useTauriEvent<string>(TauriEvent.MENU, async ({ payload }) => {
      switch (payload) {
        case 'Select Download Folder':
          {
            const directory = await dialogOpen({
              directory: true,
              title: 'Select Download Folder'
            });
            if (typeof directory === 'string') {
              DOWNLOAD_FOLDER.set(directory);
            }
          }
          break;
        case 'Open Download Folder': {
          // windows only
          await shellOpen($DOWNLOAD_FOLDER);
        }
        default:
          return;
      }
    });
  });

  onDestroy(() => {
    cleanUpSelectDownloadFolder?.();
  });
</script>

<div id="app">
  <Nav />
  <main><section><slot /></section></main>
</div>

<style lang="postcss">
  div#app {
    @apply flex;
  }

  main {
    @apply h-screen pl-16;
  }

  section {
    @apply h-full w-full overflow-y-auto px-4 scrollbar-thin scrollbar-track-white scrollbar-thumb-slate-200 hover:scrollbar-thumb-slate-400;
  }
</style>
