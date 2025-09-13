import { Component, Define, html, request } from "jay-cm";
import { Input } from "../components/input";

@Define
class Index extends Component {
  runRecipe = async () => {
    const data = this.queryAll<Input>("cm-input");
    const res = this.query<HTMLElement>("#result");
    if (!data || !res) return;

    for (const t of data) {
      if (!t.value()) {
        res.style.color = "var(--red100)";
        res.innerHTML = "Please include all start times";
        return;
      }
    }

    // Test
    await request<string>("/schedule", "POST", {
      run: true,
      time: "1 * * * * *",
      threshold_1: parseFloat(data[0].value()),
      threshold_2: parseFloat(data[1].value()),
      threshold_3: parseFloat(data[2].value()),
      threshold_4: parseFloat(data[3].value()),
    }).then(r => {
      r.ok && r.data ? (res.innerHTML = r.data) : r.error;
    });
  };

  view = html`
    <div tokens="flex items-center justify-center bg-black10 w-full h-100vh ">
      <div
        tokens="flex flex-col gap-20 p-20 bg-white rounded-2xl shadow-xl max-w-2xl"
      >
        <h1 tokens="font-bold text-black100 text-5xl mb-2">Recipe Settings</h1>
        <p tokens="text-black60 mb-4 text-base">
          To run the automatic recipe program, add all the temperature
          thresholds for each recipe. If the program is stopped, the recipes
          will need to be recalibrated.
        </p>

        <div tokens="flex gap-20 pb-10">
          <cm-input
            .label="6:00 Start"
            .placeholder="Usually 9°"
            .type="number"
          ></cm-input>

          <cm-input
            .label="4:30 Start"
            .placeholder="Usually 5°"
            .type="number"
          ></cm-input>
        </div>

        <div tokens="flex gap-20 pb-20">
          <cm-input
            .label="3:30 Start"
            .placeholder="Usually 1°"
            .type="number"
          ></cm-input>

          <cm-input
            .label="2:00 Start"
            .placeholder="Usually -3°"
            .type="number"
          ></cm-input>
        </div>

        <div tokens="flex gap-20">
          <cm-button
            .text="Run Recipe Program"
            .fill="true"
            .onclick="runRecipe"
          ></cm-button>
          <cm-button
            .text="Stop Recipe Program"
            .fill="true"
            .stop="true"
          ></cm-button>
        </div>

        <p id="result"></p>

        <div tokens="flex gap-20 justify-center items-center">
          <p tokens="text-sm text-black80">Dark Mode</p>
          <div tokens="relative inline-block w-60 h-34 bg-blue10 rounded-full">
            <cm-toggle></cm-toggle>
          </div>
        </div>
      </div>
    </div>
  `;
}
