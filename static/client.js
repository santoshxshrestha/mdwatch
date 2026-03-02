function setThemeIcon(theme) {
  const button = document.querySelector(".theme-toggle");
  if (!button) return;
  button.innerHTML =
    theme === "light"
      ? '<i class="fa-regular fa-sun"></i>'
      : '<i class="fa-solid fa-moon"></i>';
}

function toggleTheme() {
  const html = document.documentElement;
  const isLight = html.getAttribute("data-theme") === "light";

  if (isLight) {
    html.removeAttribute("data-theme");
    localStorage.setItem("theme", "dark");
    setThemeIcon("dark");
  } else {
    html.setAttribute("data-theme", "light");
    localStorage.setItem("theme", "light");
    setThemeIcon("light");
  }
}

// Load saved theme on page load
document.addEventListener("DOMContentLoaded", function () {
  const savedTheme = localStorage.getItem("theme");
  const html = document.documentElement;

  if (savedTheme === "light") {
    html.setAttribute("data-theme", "light");
    setThemeIcon("light");
  } else {
    setThemeIcon("dark");
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
};

ws.onclose = () => console.log("closed");
ws.onerror = () => console.log("error");
