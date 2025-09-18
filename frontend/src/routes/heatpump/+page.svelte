<script lang="ts">
  import { error } from "@sveltejs/kit";

  let { temp, result } = $props();

  async function getTemperature() {
    await fetch("/temp")
      .then(resp => resp.json())
      .then(data => (temp = data))
      .catch(error => console.log(error));
  }

  async function runRecipe() {
    const data = document.querySelectorAll<HTMLInputElement>("input");
    if (!data) {
      return;
    }

    for (const t of data) {
      if (!t.value) {
        result.style.color = "red";
        result.innerHTML = "Please include all start times";
        return;
      }
    }

    const response = await fetch("/schedule", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        run: true,
        time: "59 11 * * *",
        threshold_1: parseFloat(data[0].value),
        threshold_2: parseFloat(data[1].value),
        threshold_3: parseFloat(data[2].value),
        threshold_4: parseFloat(data[3].value),
      }),
    });

    if (!response.ok) {
      throw new Error(`Rsponse status: ${response.status}`);
    }

    response.json().then(r => (result = r));
  }
</script>

<h1>Heat Pump Settings</h1>

<button onclick={async () => getTemperature()}> Get temp </button>
<p>{temp}</p>

<div class="container">
  6.00 Start <input placeholder="Usually 9°" />
  4.30 Start <input placeholder="Usually 5°" />
  3.30 Start <input placeholder="Usually 1°" />
  2.00 Start <input placeholder="Usually -3°" />
</div>

<button onclick={async () => runRecipe()}> Run Recipe Program </button>

<p>{result}</p>

<style>
  .container {
    display: flex;
    flex-direction: column;
    gap: 10px;
    max-width: 500px;
    padding-bottom: 20px;
  }
</style>
