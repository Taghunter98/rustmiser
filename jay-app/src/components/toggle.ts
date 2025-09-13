import { Component, Define, Hook, html } from "jay-cm";

@Define
export class Toggle extends Component {
  view = html`
    <div
      tokens="host-auto relative inline-block w-60 h-34 bg-blue10 rounded-full"
    >
      <input id="theme-toggle" type="checkbox" tokens="appearance-none" />
      <label
        for="theme-toggle"
        tokens="absolute h-26 w-26 left-4 bottom-4 bg-blue60 transition duration-0.4s ease-in-out rounded-full cursor-pointer peer-input-checked:bg-blue100 peer-input-focus:shadow-md peer-input-checked:translate-x-26"
      >
      </label>
    </div>
  `;

  onRender = () => {
    const toggle = this.query<HTMLInputElement>("#theme-toggle");

    if (!toggle) return;

    this.onChange(toggle, () => {
      const current = document.documentElement.getAttribute("data-theme");
      const next = current === "dark" ? "light" : "dark";
      document.documentElement.setAttribute("data-theme", next);
      localStorage.setItem("theme", next);
    });

    const saved = localStorage.getItem("theme");
    if (saved === "dark") toggle.checked = true;
    if (saved) {
      document.documentElement.setAttribute("data-theme", saved);
    }
  };
}
