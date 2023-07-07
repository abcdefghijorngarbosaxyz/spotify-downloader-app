<script lang="ts">
  import { LocalKeys } from '@app/constants';
  import Main from '@app/lib/components/window/Main.svelte';
  import { useLocalStorage } from '@app/lib/utils/useLocalStorage';
  import { DOWNLOAD_FOLDER } from '@app/store';

  $: {
    $DOWNLOAD_FOLDER;
    useLocalStorage('set', LocalKeys.DOWNLOAD_FOLDER, $DOWNLOAD_FOLDER);
  }
</script>

<Main>
  <slot />
</Main>
