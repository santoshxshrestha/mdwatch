async function renderMermaid() {
  mermaid.initialize({ startOnLoad: false });

  const elements = document.querySelectorAll(".mermaid");

  for (const [index, element] of elements.entries()) {
    const graphDefinition = element.textContent;
    const { svg } = await mermaid.render(`graphDiv-${index}`, graphDefinition);
    element.innerHTML = svg;
  }
}

function setThemeIcon(theme) {
  const button = document.querySelector(".theme-toggle");
  if (!button) return;

  button.innerHTML =
    theme === "light"
      ? `
        <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" class="size-6">
          <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386-1.591 1.591M21 12h-2.25m-.386 6.364-1.591-1.591M12 18.75V21m-4.773-4.227-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z" />
        </svg>
        `
      : `
        <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" class="size-6">
          <path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.72 9.72 0 0 1 18 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 0 0 3 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 0 0 9.002-5.998Z" />
        </svg>
    `;
}

function syncHighlightTheme(theme) {
  const dark = document.getElementById("hljs-theme-dark");
  const light = document.getElementById("hljs-theme-light");
  if (!dark || !light) return;
  if (theme === "light") {
    dark.disabled = true;
    light.disabled = false;
  } else {
    dark.disabled = false;
    light.disabled = true;
  }
}

function toggleTheme() {
  const html = document.documentElement;
  const isLight = html.getAttribute("data-theme") === "light";

  if (isLight) {
    html.removeAttribute("data-theme");
    localStorage.setItem("theme", "dark");
    setThemeIcon("dark");
    syncHighlightTheme("dark");
  } else {
    html.setAttribute("data-theme", "light");
    localStorage.setItem("theme", "light");
    setThemeIcon("light");
    syncHighlightTheme("light");
  }
}

// Load saved theme on page load
document.addEventListener("DOMContentLoaded", function () {
  const savedTheme = localStorage.getItem("theme");
  const html = document.documentElement;
  renderMermaid();
  hljs.highlightAll();

  if (savedTheme === "light") {
    html.setAttribute("data-theme", "light");
    setThemeIcon("light");
    syncHighlightTheme("light");
  } else {
    setThemeIcon("dark");
    syncHighlightTheme("dark");
  }
});

// Add smooth scrolling for anchor links
document.addEventListener("click", function (e) {
  if (
    e.target.tagName === "A" &&
    e.target.getAttribute("href").startsWith("#")
  ) {
    e.preventDefault();
    const target = document.querySelector(e.target.getAttribute("href"));
    if (target) {
      target.scrollIntoView({ behavior: "smooth" });
    }
  }
});

const proto = location.protocol === "https:" ? "wss" : "ws";
const ws = new WebSocket(`${proto}://${location.host}/ws`);

ws.onopen = () => {
  console.log("connected");
};

ws.onmessage = (event) => {
  const date = new Date();
  console.log(`updating content: ${date.toLocaleTimeString()}`);
  content.innerHTML = event.data;
  hljs.highlightAll();
  renderMermaid();
};

ws.onclose = () => self.close();
ws.onerror = () => console.log("error");
