import { Component, Define, html } from "jay-cm";

@Define
export class Button extends Component {
  text: string = "Button";
  fill: boolean = false;
  stop: boolean = false;

  view = () => html` <button
    tokens="
            ${this.fill ? "w-full" : "host-auto"}
            font-medium text-lg pl-22 pr-22 pt-9 pb-9 text-white ${this.stop
      ? "bg-red100 hover:bg-red80 active:bg-red60"
      : "bg-blue100 hover:bg-blue80 active:bg-blue60"} rounded-md cursor-pointer
            transition-all hover:scale-95% duration-0.1s ease-in-out
        "
  >
    ${this.text}
  </button>`;
}
