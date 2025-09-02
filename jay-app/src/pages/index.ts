import { Component, Define, html } from "jay-cm";
import { Input } from "../components/input";

@Define
class Index extends Component {
  text = "Asgard Heat Pump";

  view = html`
    <div tokens="flex items-center justify-center bg-black10 w-full h-100vh ">
      <div tokens="flex flex-col gap-20 p-20 bg-white rounded-2xl shadow-lg">
        <h1 tokens="font-bold text-black100 text-5xl mb-2">${this.text}</h1>
        <p tokens="text-black60 mb-4 text-base">
          Set the time for the system to run. To stop press the stop button.
        </p>

        <cm-input
          .label="Set Time"
          .placeholder="Select a time for the program to run"
        ></cm-input>

        <div tokens="flex gap-20">
          <cm-button .text="Run Recipe Program" .fill="true"></cm-button>
          <cm-button
            .text="Stop Recipe Program"
            .fill="true"
            .stop="true"
          ></cm-button>
        </div>

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
