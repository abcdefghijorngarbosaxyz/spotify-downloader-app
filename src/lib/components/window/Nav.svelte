<script lang="ts">
  import type { Page } from '@app/types';

  import { Search, Queue, Options } from '../icons';

  const Pages: Page = {
    '/': {
      name: 'Search',
      icon: Search
    },
    '/Downloads': {
      name: 'Downloads',
      icon: Queue
    },
    '/Options': {
      name: 'Options',
      icon: Options
    }
  };
</script>

<nav>
  {#each Object.entries(Pages) as [key, { name, icon }]}
    <a href={key}>
      <div><svelte:component this={icon} class="w-5 h-5" /></div>
      <p>{name}</p>
    </a>
  {/each}
</nav>

<style lang="postcss" scoped>
  nav {
    @apply fixed left-0 top-0 h-screen w-12 transform space-y-[1px] overflow-hidden border-r bg-slate-200 hover:w-56 hover:shadow hover:delay-500;
  }
  a {
    @apply flex hover:bg-slate-400/50;
  }
  a > div {
    @apply flex h-12 w-12 flex-none items-center justify-center;
  }
  a > p {
    @apply flex items-center;
  }
</style>
