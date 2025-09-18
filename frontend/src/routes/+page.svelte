<script lang="ts">
  import { onMount } from "svelte";
  let files: string[] = [];

  onMount(async () => {
    try {
      const res = await fetch("/files");
      const data = await res.json();
      files = data.files;
    } catch (err) {
      console.error("Failed to load files:", err);
    }
  });
</script>

<h1>FX FTP Server</h1>
<p>Current files</p>

{#if files.length > 0}
  <ul>
    {#each files as file}
      <li>
        <a href={file} target="_blank" rel="noopener"
          >{file.replace("/uploads/", "")}</a
        >
      </li>
    {/each}
  </ul>
{:else}
  <p>No files found.</p>
{/if}
