<script lang="ts">
  let result = $state();

  async function handleSubmit(event: Event) {
    event.preventDefault();

    const form = event.currentTarget as HTMLFormElement;
    const input = form.querySelector('input[name="file"]') as HTMLInputElement;

    if (!input.files) return;

    const formData = new FormData();

    for (const file of input.files) {
      formData.append("file", file);
    }

    try {
      const res = await fetch("/upload", {
        method: "POST",
        body: formData,
      });

      if (res.ok) {
        result = "Files uploaded successfully";
      } else {
        result = "Upload failed" + (await res.text());
      }
    } catch (err) {
      result = "Error during upload " + err;
    }
  }
</script>

<form onsubmit={handleSubmit}>
  <input name="file" type="file" multiple />
  <button type="submit">Upload Files</button>
</form>

{result}
