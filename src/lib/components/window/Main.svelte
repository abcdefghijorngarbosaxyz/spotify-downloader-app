<script>
  import Nav from './Nav.svelte';
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
