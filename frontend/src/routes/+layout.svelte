<script lang="ts">
  import favicon from "$lib/assets/favicon.svg";

  let { children } = $props();
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
</svelte:head>

<nav>
  <a href="/">Home</a>
  <a href="/upload">Upload Files</a>
  <a href="/heatpump">Heatpump</a>
</nav>

{@render children?.()}
