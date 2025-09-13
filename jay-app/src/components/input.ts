import { Component, Define, html } from "jay-cm";

@Define
export class Input extends Component {
  placeholder: string = "Enter Something";
  label: string = "Label";
  type: string = "text";

  view = () => html`
    <label>${this.label}</label>
    <input
      tokens="w-full mt-10 pl-10 pr-10 pt-8 pb-8 border-solid border-black10 rounded-md
      hover:border-blue80 focus:outline-none focus:border-blue100
      "
      placeholder="${this.placeholder}"
      type="${this.type}"
    />
  `;

  value = () => this.query<HTMLInputElement>("input")?.value || "";
}
