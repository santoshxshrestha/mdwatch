use askama::Template;
#[derive(Template)]
#[template(
    ext = "html",
    source = "
<!doctype html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\" />
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
    <title>{{ title }} - mdwatch</title>
    <style>
:root {
    --bg-color: #0d1117;
    --text-color: #e6edf3;
    --heading-color: #f0f6fc;
    --code-bg: #161b22;
    --code-color: #e6edf3;
    --code-border: #30363d;
    --quote-bg: #0d1117;
    --quote-border: #656d76;
    --quote-text: #8b949e;
    --link-color: #2f81f7;
    --link-hover: #58a6ff;
    --button-bg: #21262d;
    --button-color: #f0f6fc;
    --button-hover: #30363d;
    --border-color: #30363d;
    --table-border: #30363d;
    --table-header-bg: #161b22;
    --table-row-bg: #0d1117;
    --table-row-hover: #161b22;
    --hr-color: #30363d;
    --selection-bg: #264f78;
}

[data-theme=\"light\"] {
    --bg-color: #ffffff;
    --text-color: #1f2328;
    --heading-color: #1f2328;
    --code-bg: #f6f8fa;
    --code-color: #1f2328;
    --code-border: #d1d9e0;
    --quote-bg: #f6f8fa;
    --quote-border: #d1d9e0;
    --quote-text: #656d76;
    --link-color: #0969da;
    --link-hover: #0550ae;
    --button-bg: #f6f8fa;
    --button-color: #1f2328;
    --button-hover: #f3f4f6;
    --border-color: #d1d9e0;
    --table-border: #d1d9e0;
    --table-header-bg: #f6f8fa;
    --table-row-bg: #ffffff;
    --table-row-hover: #f6f8fa;
    --hr-color: #d1d9e0;
    --selection-bg: #0969da20;
}

* {
    box-sizing: border-box;
}

body {
    max-width: 980px;
    margin: 0 auto;
    padding: 45px;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Noto Sans', Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji';
    font-size: 16px;
    line-height: 1.5;
    background-color: var(--bg-color);
    color: var(--text-color);
    transition: background-color 0.2s ease, color 0.2s ease;
}

::selection {
    background: var(--selection-bg);
}

h1, h2, h3, h4, h5, h6 {
    margin-top: 24px;
    margin-bottom: 16px;
    font-weight: 600;
    line-height: 1.25;
    color: var(--heading-color);
}

h1 {
    font-size: 2em;
    margin-bottom: 16px;
    padding-bottom: 10px;
    border-bottom: 1px solid var(--border-color);
}

h2 {
    font-size: 1.5em;
    margin-bottom: 16px;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--border-color);
}

h3 {
    font-size: 1.25em;
}

h4 {
    font-size: 1em;
}

h5 {
    font-size: 0.875em;
}

h6 {
    font-size: 0.85em;
    color: var(--quote-text);
}

p {
    margin-top: 0;
    margin-bottom: 16px;
}

code {
    background: var(--code-bg);
    padding: 0.2em 0.4em;
    border-radius: 6px;
    font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, 'Liberation Mono', monospace;
    font-size: 85%;
    color: var(--code-color);
    border: 1px solid var(--code-border);
}

pre {
    background: var(--code-bg);
    padding: 16px;
    overflow: auto;
    border-radius: 6px;
    color: var(--code-color);
    border: 1px solid var(--code-border);
    margin-top: 0;
    margin-bottom: 16px;
    font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, 'Liberation Mono', monospace;
    font-size: 85%;
    line-height: 1.45;
}

pre code {
    background: transparent;
    border: none;
    padding: 0;
    font-size: 100%;
    color: inherit;
}

blockquote {
    border-left: 4px solid var(--quote-border);
    padding: 0 1em;
    margin: 0 0 16px 0;
    color: var(--quote-text);
    background-color: var(--quote-bg);
    border-radius: 0 6px 6px 0;
}

blockquote > :first-child {
    margin-top: 0;
}

blockquote > :last-child {
    margin-bottom: 0;
}

a {
    color: var(--link-color);
    text-decoration: none;
}

a:hover {
    color: var(--link-hover);
    text-decoration: underline;
}

ul, ol {
    padding-left: 2em;
    margin-top: 0;
    margin-bottom: 16px;
}

li + li {
    margin-top: 0.25em;
}

table {
    border-collapse: collapse;
    border-spacing: 0;
    width: 100%;
    margin-top: 0;
    margin-bottom: 16px;
}

th, td {
    padding: 6px 13px;
    border: 1px solid var(--table-border);
    text-align: left;
}

th {
    background-color: var(--table-header-bg);
    font-weight: 600;
}

tr {
    background-color: var(--table-row-bg);
}

tr:nth-child(2n) {
    background-color: var(--table-row-hover);
}

hr {
    height: 0.25em;
    padding: 0;
    margin: 24px 0;
    background-color: var(--hr-color);
    border: 0;
    border-radius: 2px;
}

img {
    max-width: 100%;
    height: auto;
    border-radius: 6px;
}

.task-list-item {
    list-style-type: none;
    margin-left: -1.5em;
}

.task-list-item-checkbox {
    margin-right: 0.5em;
}

strong {
    font-weight: 600;
}

em {
    font-style: italic;
}

.theme-toggle {
    position: fixed;
    top: 20px;
    right: 20px;
    background: var(--button-bg);
    color: var(--button-color);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    width: 40px;
    height: 40px;
    font-size: 16px;
    cursor: pointer;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.12), 0 1px 2px rgba(0, 0, 0, 0.24);
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
}

.theme-toggle:hover {
    background: var(--button-hover);
    box-shadow: 0 3px 6px rgba(0, 0, 0, 0.16), 0 3px 6px rgba(0, 0, 0, 0.23);
}

.theme-toggle:active {
    transform: scale(0.95);
}

@media (max-width: 767px) {
    body {
        padding: 15px;
    }

    .theme-toggle {
        top: 15px;
        right: 15px;
    }
}

.highlight {
    background: var(--code-bg);
    border-radius: 6px;
    padding: 16px;
    margin: 16px 0;
    border: 1px solid var(--code-border);
}

details {
    margin-bottom: 16px;
}

summary {
    cursor: pointer;
    font-weight: 600;
    padding: 8px 0;
}

details[open] summary {
    border-bottom: 1px solid var(--border-color);
    margin-bottom: 8px;
}

.footnote-ref {
    vertical-align: super;
    font-size: 0.8em;
}

.footnote-definition {
    border-top: 1px solid var(--border-color);
    padding-top: 8px;
    margin-top: 16px;
    font-size: 0.9em;
}

kbd {
    background-color: var(--code-bg);
    border: 1px solid var(--code-border);
    border-radius: 3px;
    box-shadow: inset 0 -1px 0 var(--code-border);
    color: var(--code-color);
    display: inline-block;
    font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, 'Liberation Mono', monospace;
    font-size: 11px;
    line-height: 10px;
    padding: 3px 5px;
    vertical-align: middle;
}

.alert {
    padding: 16px;
    margin-bottom: 16px;
    border-left: 4px solid var(--link-color);
    border-radius: 0 6px 6px 0;
    background-color: var(--quote-bg);
}

.alert-info {
    border-left-color: var(--link-color);
}

.alert-warning {
    border-left-color: #fb8500;
}

.alert-danger {
    border-left-color: #dc3545;
}

.alert-success {
    border-left-color: #28a745;
}
</style>
</head>
<body>
<button class=\"theme-toggle\" onclick=\"toggleTheme()\" title=\"Toggle theme\">
        🌙
    </button>
    <article id=\"content\">{{content | safe}}</article>

    <script>
        function toggleTheme() {
            const html = document.documentElement;
            const button = document.querySelector('.theme-toggle');

            if (html.getAttribute('data-theme') === 'light') {
                html.removeAttribute('data-theme');
                button.textContent = '🌙';
                localStorage.setItem('theme', 'dark');
            } else {
                html.setAttribute('data-theme', 'light');
                button.textContent = '☀️';
                localStorage.setItem('theme', 'light');
            }
        }

        // Load saved theme on page load
        document.addEventListener('DOMContentLoaded', function() {
            const savedTheme = localStorage.getItem('theme');
            const html = document.documentElement;
            const button = document.querySelector('.theme-toggle');

            if (savedTheme === 'light') {
                html.setAttribute('data-theme', 'light');
                button.textContent = '☀️';
            } else {
                button.textContent = '🌙';
            }
        });

        let lastSeen = Number(sessionStorage.getItem(\"lastSeen\") || 0);

        setInterval(() => {
            fetch(\"/api/check-update\")
                .then(res => res.json())
                .then(data => {
                    if (data.last_modified > lastSeen) {
                        sessionStorage.setItem(\"lastSeen\", data.last_modified);
                        location.reload();
                    }
                });
        }, 1000);

        // Add smooth scrolling for anchor links
        document.addEventListener('click', function(e) {
            if (e.target.tagName === 'A' && e.target.getAttribute('href').startsWith('#')) {
                e.preventDefault();
                const target = document.querySelector(e.target.getAttribute('href'));
                if (target) {
                    target.scrollIntoView({ behavior: 'smooth' });
                }
            }
        });
    </script>
</body>
</html>
"
)]
pub struct Home {
    pub content: String,
    pub last_modified: u64,
    pub title: String,
}
