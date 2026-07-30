use pulldown_cmark::{Options, Parser, html};

/// Split a leading YAML front-matter block from the document. The block must
/// start with a `---` line at the very top and end at the next `---` or `...`
/// line. Returns `(frontmatter, body)`; `None` when there is no such block (so
/// a lone `---` stays a normal thematic break).
fn split_frontmatter(markdown: &str) -> Option<(&str, &str)> {
    let src = markdown.strip_prefix('\u{feff}').unwrap_or(markdown);

    let first_nl = src.find('\n')?;
    if src[..first_nl].trim_end_matches('\r') != "---" {
        return None;
    }
    let content_start = first_nl + 1;

    let mut cursor = content_start;
    loop {
        let (line, next, at_eof) = match src[cursor..].find('\n') {
            Some(p) => (src[cursor..cursor + p].trim_end_matches('\r'), cursor + p + 1, false),
            None => (src[cursor..].trim_end_matches('\r'), src.len(), true),
        };
        if line == "---" || line == "..." {
            let body = if next <= src.len() { &src[next..] } else { "" };
            return Some((&src[content_start..cursor], body));
        }
        if at_eof {
            return None; // no closing delimiter -> not front matter
        }
        cursor = next;
    }
}

fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(c),
        }
    }
    out
}

/// Render captured front matter as a dimmed, comment-style block.
fn render_frontmatter(frontmatter: &str) -> String {
    let normalized = frontmatter.replace("\r\n", "\n");
    let trimmed = normalized.trim_matches('\n');
    if trimmed.trim().is_empty() {
        return String::new();
    }
    format!(
        "<pre class=\"mdview-frontmatter\">{}</pre>\n",
        escape_html(trimmed)
    )
}

pub fn markdown_to_html(markdown: &str) -> String {
    let (frontmatter_html, body) = match split_frontmatter(markdown) {
        Some((fm, rest)) => (render_frontmatter(fm), rest),
        None => (String::new(), markdown),
    };

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(body, options);

    let mut html_output = String::new();
    html_output.push_str(&frontmatter_html);
    html::push_html(&mut html_output, parser);
    html_output
}

/// CSS for the rendered document. Color placeholders (`%%NAME%%`) are substituted
/// in `wrap_html` so the stylesheet can stay a plain string literal (no `format!`
/// brace escaping).
const STYLE_TEMPLATE: &str = r#"
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
    font-size: 14px;
    line-height: 1.6;
    padding: 20px;
    max-width: 900px;
    margin: 0 auto;
    background-color: %%BG%%;
    color: %%TEXT%%;
}
a { color: %%LINK%%; text-decoration: none; }
a:hover { text-decoration: underline; }
code {
    background-color: %%CODE_BG%%;
    padding: 0.2em 0.4em;
    border-radius: 3px;
    font-family: "Cascadia Code", "Fira Code", Consolas, monospace;
    font-size: 85%;
}
pre {
    background-color: %%CODE_BG%%;
    padding: 16px;
    overflow: auto;
    border-radius: 6px;
}
pre code {
    background-color: transparent;
    padding: 0;
}
blockquote {
    border-left: 4px solid %%BORDER%%;
    margin: 0;
    padding-left: 16px;
    color: %%TEXT%%;
    opacity: 0.8;
}
table {
    border-collapse: collapse;
    width: 100%;
}
th, td {
    border: 1px solid %%BORDER%%;
    padding: 8px 12px;
    text-align: left;
}
th {
    background-color: %%CODE_BG%%;
}
img {
    max-width: 100%;
}
h1, h2 {
    border-bottom: 1px solid %%BORDER%%;
    padding-bottom: 0.3em;
}
hr {
    border: none;
    border-top: 1px solid %%BORDER%%;
}
input[type="checkbox"] {
    margin-right: 0.5em;
}
a { cursor: pointer; }

/* YAML front matter, shown dimmed like a comment. */
.mdview-frontmatter {
    background-color: transparent;
    color: %%MUTED%%;
    border-left: 3px solid %%BORDER%%;
    border-radius: 0;
    margin: 0 0 18px 0;
    padding: 8px 12px;
    font-size: 12px;
    line-height: 1.45;
    white-space: pre-wrap;
    word-break: break-word;
    overflow-x: auto;
}

/* ---- Table of contents ---- */
#mdview-toc {
    position: fixed;
    top: 0;
    left: 0;
    width: 280px;
    height: 100vh;
    box-sizing: border-box;
    overflow-y: auto;
    padding: 44px 8px 24px 12px;
    background-color: %%TOC_BG%%;
    border-right: 1px solid %%BORDER%%;
    font-size: 13px;
    line-height: 1.35;
    z-index: 1000;
    transition: transform 0.15s ease;
    display: none;
}
#mdview-toc.mdview-toc-hidden {
    transform: translateX(-100%);
}
#mdview-toc-header {
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    font-size: 11px;
    opacity: 0.6;
    padding: 0 6px 8px 6px;
}
#mdview-toc ul {
    list-style: none;
    margin: 0;
    padding: 0;
}
#mdview-toc li { margin: 0; padding: 0; }
#mdview-toc .mdview-toc-row {
    display: flex;
    align-items: baseline;
    border-radius: 4px;
}
#mdview-toc .mdview-toc-row:hover { background-color: %%TOC_HOVER%%; }
#mdview-toc .mdview-toc-caret {
    flex: 0 0 auto;
    width: 16px;
    text-align: center;
    cursor: pointer;
    user-select: none;
    opacity: 0.55;
    font-size: 10px;
}
#mdview-toc .mdview-toc-caret.mdview-toc-empty {
    visibility: hidden;
    cursor: default;
}
#mdview-toc a {
    flex: 1 1 auto;
    display: block;
    padding: 3px 6px 3px 2px;
    color: %%TEXT%%;
    text-decoration: none;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}
#mdview-toc a:hover { text-decoration: none; }
#mdview-toc a.mdview-toc-active {
    color: %%LINK%%;
    font-weight: 600;
    background-color: %%TOC_ACTIVE%%;
    border-radius: 4px;
}
#mdview-toc li.mdview-toc-collapsed > ul { display: none; }

#mdview-toc-toggle {
    position: fixed;
    top: 8px;
    left: 8px;
    z-index: 1002;
    width: 32px;
    height: 32px;
    box-sizing: border-box;
    border: 1px solid %%BORDER%%;
    border-radius: 6px;
    background-color: %%TOC_BG%%;
    color: %%TEXT%%;
    cursor: pointer;
    font-size: 15px;
    line-height: 1;
    padding: 0;
    display: none;
    align-items: center;
    justify-content: center;
    transition: left 0.15s ease;
}
#mdview-toc-toggle:hover { background-color: %%TOC_HOVER%%; }
body.mdview-toc-open #mdview-toc-toggle { left: 248px; }

/* Reserve space for the panel when open so it never overlaps the content. */
body.mdview-toc-open { padding-left: 300px; }

/* ---- Find bar (/) ---- */
#mdview-find {
    position: fixed;
    top: 10px;
    right: 14px;
    z-index: 1003;
    display: none;
    align-items: center;
    gap: 8px;
    padding: 6px 8px;
    background-color: %%TOC_BG%%;
    border: 1px solid %%BORDER%%;
    border-radius: 6px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.18);
}
#mdview-find input {
    border: 1px solid %%BORDER%%;
    border-radius: 4px;
    padding: 3px 6px;
    font-size: 13px;
    background-color: %%BG%%;
    color: %%TEXT%%;
    outline: none;
    min-width: 180px;
}
#mdview-find-status {
    font-size: 12px;
    opacity: 0.7;
    min-width: 52px;
    white-space: nowrap;
}
"#;

/// Client-side script: heading ids, hash navigation, the collapsible TOC,
/// scroll-spy, scroll preservation across refresh, and keyboard shortcuts.
/// Plain string literal (no interpolation) so braces need no escaping.
const VIEWER_SCRIPT: &str = r#"
(function() {
    function slugify(text) {
        return (text || '')
            .toLowerCase()
            .trim()
            .replace(/[\s]+/g, '-')
            .replace(/[^a-z0-9\-_]/g, '-')
            .replace(/-+/g, '-')
            .replace(/^-|-$/g, '');
    }

    function assignHeadingIds() {
        var seen = Object.create(null);
        document.querySelectorAll('h1,h2,h3,h4,h5,h6').forEach(function(heading) {
            if (heading.id) { seen[heading.id] = true; return; }

            var baseId = slugify(heading.textContent);
            if (!baseId) baseId = 'section';

            var id = baseId;
            var counter = 1;
            while (seen[id] || document.getElementById(id)) {
                counter += 1;
                id = baseId + '-' + counter;
            }

            seen[id] = true;
            heading.id = id;
        });
    }

    function scrollToHash(hash) {
        if (!hash || hash.charAt(0) !== '#') return false;

        var id = decodeURIComponent(hash.slice(1));
        if (!id) return false;

        var target = document.getElementById(id);
        if (!target) return false;

        target.scrollIntoView();
        if (history && history.replaceState) {
            history.replaceState(null, '', hash);
        }
        return true;
    }

    // ---- Persisted state ----
    // localStorage (not sessionStorage): Total Commander recreates the Lister
    // window on focus/selection change (ListCloseWindow + ListLoad), which
    // starts a fresh WebView2 session. localStorage lives in the shared user-
    // data profile, so scroll / TOC / zoom state survives that recreation.
    function scrollKey() { return 'mdview:scroll:' + (location.pathname || ''); }
    function tocStateKey() { return 'mdview:toc:' + (location.pathname || ''); }
    var TOC_OPEN_KEY = 'mdview:tocOpen';
    var ZOOM_KEY = 'mdview:zoom';

    function saveScroll() {
        try {
            localStorage.setItem(scrollKey(), String(window.scrollY || window.pageYOffset || 0));
        } catch (e) {}
    }

    function restoreScroll() {
        try {
            var v = localStorage.getItem(scrollKey());
            if (v !== null) { window.scrollTo(0, parseInt(v, 10) || 0); }
        } catch (e) {}
    }

    // ---- Zoom: `+`/`-` step by 10%, `z` resets to 120% ----
    function getZoom() {
        var z = parseFloat(document.documentElement.style.zoom);
        return (isNaN(z) || z <= 0) ? 1 : z;
    }
    function applyZoom(z) {
        z = Math.max(0.5, Math.min(3, Math.round(z * 100) / 100));
        document.documentElement.style.zoom = String(z);
        try { localStorage.setItem(ZOOM_KEY, String(z)); } catch (e) {}
    }
    function restoreZoom() {
        try {
            var v = localStorage.getItem(ZOOM_KEY);
            if (v) document.documentElement.style.zoom = String(parseFloat(v) || 1);
        } catch (e) {}
    }

    // ---- Table of contents ----
    function buildToc() {
        var panel = document.getElementById('mdview-toc');
        var toggle = document.getElementById('mdview-toc-toggle');
        var listRoot = document.getElementById('mdview-toc-list');
        if (!panel || !listRoot) return null;

        var headings = Array.prototype.slice.call(
            document.querySelectorAll('h1,h2,h3,h4,h5,h6')
        ).filter(function(h) { return h.id; });

        if (!headings.length) {
            panel.style.display = 'none';
            if (toggle) toggle.style.display = 'none';
            return null;
        }

        var root = { level: 0, children: [] };
        var stack = [root];
        headings.forEach(function(h) {
            var level = parseInt(h.tagName.substring(1), 10);
            while (stack.length > 1 && stack[stack.length - 1].level >= level) {
                stack.pop();
            }
            var node = { level: level, heading: h, children: [] };
            stack[stack.length - 1].children.push(node);
            stack.push(node);
        });

        var linkMap = Object.create(null);

        function render(node, ul) {
            node.children.forEach(function(child) {
                var li = document.createElement('li');
                li.setAttribute('data-heading', child.heading.id);

                var row = document.createElement('div');
                row.className = 'mdview-toc-row';
                row.style.paddingLeft = ((child.level - 1) * 14) + 'px';

                var caret = document.createElement('span');
                caret.className = 'mdview-toc-caret';
                if (child.children.length) {
                    caret.textContent = '▾';
                    caret.addEventListener('click', function(e) {
                        e.preventDefault();
                        e.stopPropagation();
                        var collapsed = li.classList.toggle('mdview-toc-collapsed');
                        caret.textContent = collapsed ? '▸' : '▾';
                        saveTocState();
                    });
                } else {
                    caret.className += ' mdview-toc-empty';
                }
                row.appendChild(caret);

                var a = document.createElement('a');
                a.href = '#' + child.heading.id;
                a.textContent = child.heading.textContent || '(untitled)';
                a.title = a.textContent;
                row.appendChild(a);
                linkMap[child.heading.id] = a;

                li.appendChild(row);

                if (child.children.length) {
                    var childUl = document.createElement('ul');
                    render(child, childUl);
                    li.appendChild(childUl);
                }
                ul.appendChild(li);
            });
        }

        listRoot.innerHTML = '';
        render(root, listRoot);
        // Explicit value: '' would fall back to the stylesheet's `display: none`.
        panel.style.display = 'block';
        if (toggle) toggle.style.display = 'flex';
        return linkMap;
    }

    function saveTocState() {
        try {
            var panel = document.getElementById('mdview-toc');
            var collapsed = [];
            document.querySelectorAll('#mdview-toc li.mdview-toc-collapsed').forEach(function(li) {
                var h = li.getAttribute('data-heading');
                if (h) collapsed.push(h);
            });
            localStorage.setItem(tocStateKey(), JSON.stringify({ collapsed: collapsed }));
            var open = !!panel
                && !panel.classList.contains('mdview-toc-hidden')
                && panel.style.display !== 'none';
            localStorage.setItem(TOC_OPEN_KEY, open ? '1' : '0'); // global preference
        } catch (e) {}
    }

    function applyTocOpen(open) {
        var panel = document.getElementById('mdview-toc');
        var toggle = document.getElementById('mdview-toc-toggle');
        if (!panel) return;
        if (open) {
            panel.classList.remove('mdview-toc-hidden');
            document.body.classList.add('mdview-toc-open');
            if (toggle) toggle.textContent = '‹';
        } else {
            panel.classList.add('mdview-toc-hidden');
            document.body.classList.remove('mdview-toc-open');
            if (toggle) toggle.textContent = '☰';
        }
    }

    function restoreTocState() {
        var collapsed = [];
        try {
            var s = JSON.parse(localStorage.getItem(tocStateKey()) || 'null');
            if (s && s.collapsed) collapsed = s.collapsed;
        } catch (e) {}

        collapsed.forEach(function(hid) {
            var li = document.querySelector('#mdview-toc li[data-heading="' + hid + '"]');
            if (!li) return;
            var caret = li.querySelector(':scope > .mdview-toc-row > .mdview-toc-caret');
            if (!caret || caret.classList.contains('mdview-toc-empty')) return;
            li.classList.add('mdview-toc-collapsed');
            caret.textContent = '▸';
        });

        // Open/closed is a global preference so a `t`-minimize sticks across
        // files and across TC's focus-driven reloads. Default open.
        var open = true;
        try {
            var o = localStorage.getItem(TOC_OPEN_KEY);
            if (o !== null) open = (o === '1');
        } catch (e) {}
        applyTocOpen(open);
    }

    function toggleToc() {
        var panel = document.getElementById('mdview-toc');
        if (!panel || panel.style.display === 'none') return;
        var willOpen = panel.classList.contains('mdview-toc-hidden');
        applyTocOpen(willOpen);
        tocActive = willOpen; // opening hands j/k to the TOC; closing hands it back to the body
        saveTocState();
    }

    function setupScrollSpy(linkMap) {
        var headings = Array.prototype.slice.call(
            document.querySelectorAll('h1,h2,h3,h4,h5,h6')
        ).filter(function(h) { return h.id && linkMap[h.id]; });
        if (!headings.length) return;

        var current = null;
        var ticking = false;

        function update() {
            ticking = false;
            var offset = 100;
            var active = headings[0];
            for (var i = 0; i < headings.length; i++) {
                if (headings[i].getBoundingClientRect().top <= offset) active = headings[i];
                else break;
            }
            if (active === current) return;
            current = active;

            for (var id in linkMap) {
                linkMap[id].classList.remove('mdview-toc-active');
            }
            var link = linkMap[active.id];
            if (!link) return;
            link.classList.add('mdview-toc-active');

            var panel = document.getElementById('mdview-toc');
            if (panel && panel.style.display !== 'none'
                && !panel.classList.contains('mdview-toc-hidden')) {
                var linkTop = link.offsetTop;
                if (linkTop < panel.scrollTop
                    || linkTop > panel.scrollTop + panel.clientHeight - 30) {
                    panel.scrollTop = Math.max(0, linkTop - panel.clientHeight / 2);
                }
            }
        }

        window.addEventListener('scroll', function() {
            if (!ticking) { ticking = true; window.requestAnimationFrame(update); }
        }, { passive: true });
        update();
    }

    function initToc() {
        var linkMap = buildToc();
        if (!linkMap) return;
        restoreTocState();
        setupScrollSpy(linkMap);
        var toggle = document.getElementById('mdview-toc-toggle');
        if (toggle) {
            toggle.addEventListener('click', function(e) {
                e.preventDefault();
                toggleToc();
            });
        }
        // Track which region j/k drive, based on where the user last clicked.
        document.addEventListener('click', function(e) {
            var toc = document.getElementById('mdview-toc');
            var find = document.getElementById('mdview-find');
            if (toc && toc.contains(e.target)) tocActive = true;
            else if (!(find && find.contains(e.target))) tocActive = false;
        }, true);
    }

    // ---- Boot ----
    restoreZoom();
    assignHeadingIds();
    initToc();

    if (!(location.hash && scrollToHash(location.hash))) {
        restoreScroll();
    }

    window.addEventListener('beforeunload', saveScroll);
    setInterval(saveScroll, 1000);

    document.addEventListener('click', function(e) {
        var link = e.target.closest('a');
        var webview = window.chrome && window.chrome.webview;
        if (!link) return;

        var href = link.getAttribute('href');
        if (!href) return;

        if (href.charAt(0) === '#') {
            e.preventDefault();
            scrollToHash(href);
            return;
        }

        if (webview && e.ctrlKey) {
            e.preventDefault();
            webview.postMessage({type: 'openLink', url: link.href || href});
        }
    });

    // ---- Vim-style navigation ----
    var LINE_STEP = 64;
    var pendingG = false, gTimer = null;

    function docBottom() {
        return Math.max(
            document.body.scrollHeight,
            document.documentElement.scrollHeight
        );
    }

    function handleG(e) {
        e.preventDefault();
        if (pendingG) {
            pendingG = false;
            if (gTimer) { clearTimeout(gTimer); gTimer = null; }
            window.scrollTo(0, 0); // gg -> top
        } else {
            pendingG = true;
            gTimer = setTimeout(function() { pendingG = false; gTimer = null; }, 600);
        }
    }

    function tocVisible() {
        var el = document.getElementById('mdview-toc');
        return !!el && el.style.display !== 'none'
            && !el.classList.contains('mdview-toc-hidden');
    }

    // Which region j/k drive: the TOC (true) or the document body (false).
    // Defaults to the TOC when it is open on entry; clicking the body switches
    // to the body, clicking the TOC switches back. This runs after the boot
    // section above, so initToc() has already built and restored the TOC and
    // tocVisible() is accurate here.
    var tocActive = tocVisible();

    function visibleTocLinks() {
        return Array.prototype.slice.call(
            document.querySelectorAll('#mdview-toc a')
        ).filter(function(a) { return a.offsetParent !== null; });
    }

    function currentActiveIndex(links) {
        var active = document.querySelector('#mdview-toc a.mdview-toc-active');
        return active ? links.indexOf(active) : -1;
    }

    // Step the TOC selection to the next/prev visible entry and jump the
    // document to it. Driven by j/k while the TOC is the active region.
    function moveTocSelection(dir) {
        var links = visibleTocLinks();
        if (!links.length) return;
        var next = currentActiveIndex(links) + dir;
        if (next < 0) next = 0;
        if (next > links.length - 1) next = links.length - 1;
        var href = links[next].getAttribute('href');
        if (href && href.charAt(0) === '#') scrollToHash(href);
    }

    // Jump the document to the previous/next heading (bound to [ and ]).
    function jumpHeading(dir) {
        var hs = Array.prototype.slice.call(
            document.querySelectorAll('h1,h2,h3,h4,h5,h6')
        ).filter(function(h) { return h.id; });
        if (!hs.length) return;
        var target = null, i;
        if (dir > 0) {
            for (i = 0; i < hs.length; i++) {
                if (hs[i].getBoundingClientRect().top > 2) { target = hs[i]; break; }
            }
            if (!target) target = hs[hs.length - 1];
        } else {
            for (i = hs.length - 1; i >= 0; i--) {
                if (hs[i].getBoundingClientRect().top < -2) { target = hs[i]; break; }
            }
            if (!target) target = hs[0];
        }
        scrollToHash('#' + target.id);
    }

    // ---- Find bar (triggered by `/`) ----
    function openFind() {
        var bar = document.getElementById('mdview-find');
        if (!bar) return;
        bar.style.display = 'flex';
        var input = document.getElementById('mdview-find-input');
        if (input) { input.focus(); input.select(); }
    }

    function closeFind() {
        var bar = document.getElementById('mdview-find');
        if (bar) bar.style.display = 'none';
        var input = document.getElementById('mdview-find-input');
        if (input) input.blur();
        var status = document.getElementById('mdview-find-status');
        if (status) status.textContent = '';
    }

    function runFind(query, backwards) {
        var status = document.getElementById('mdview-find-status');
        if (!query) { if (status) status.textContent = ''; return; }
        var found = false;
        try {
            // window.find(text, caseSensitive, backwards, wrapAround, wholeWord, inFrames, showDialog)
            found = window.find(query, false, !!backwards, true, false, false, false);
        } catch (err) {}
        if (status) status.textContent = found ? '' : 'No match';
    }

    document.addEventListener('keydown', function(e) {
        var webview = window.chrome && window.chrome.webview;

        if (e.key === 'Escape') {
            if (webview) webview.postMessage({type: 'close'});
            return;
        }

        // Never hijack keys while typing in a field.
        var t = e.target;
        var editable = t && (t.isContentEditable
            || /^(input|textarea|select)$/i.test(t.tagName || ''));
        if (editable) return;

        // Ctrl+d / Ctrl+u: half-page down / up (vim / Sumatra).
        if (e.ctrlKey && !e.altKey && !e.metaKey && (e.key === 'd' || e.key === 'u')) {
            e.preventDefault();
            var half = (window.innerHeight || 600) * 0.5;
            window.scrollBy(0, e.key === 'd' ? half : -half);
            return;
        }

        // Ctrl+f / Ctrl+b: full-page down / up (vim). preventDefault also
        // suppresses the browser's built-in find on Ctrl+f.
        if (e.ctrlKey && !e.altKey && !e.metaKey && (e.key === 'f' || e.key === 'b')) {
            e.preventDefault();
            var page = (window.innerHeight || 600) * 0.9;
            window.scrollBy(0, e.key === 'f' ? page : -page);
            return;
        }

        // Everything below is a plain key (Shift is allowed, for `G`).
        if (e.ctrlKey || e.metaKey || e.altKey) return;

        switch (e.key) {
            case 'j':
                e.preventDefault();
                if (tocActive && tocVisible()) moveTocSelection(1); // navigate TOC entries
                else window.scrollBy(0, LINE_STEP);                 // otherwise scroll the doc
                return;
            case 'k':
                e.preventDefault();
                if (tocActive && tocVisible()) moveTocSelection(-1);
                else window.scrollBy(0, -LINE_STEP);
                return;
            case '[': e.preventDefault(); jumpHeading(-1); return; // previous heading
            case ']': e.preventDefault(); jumpHeading(1); return;  // next heading
            case '/': e.preventDefault(); openFind(); return;      // open find bar
            case '+': case '=': e.preventDefault(); applyZoom(getZoom() + 0.1); return; // zoom in
            case '-': case '_': e.preventDefault(); applyZoom(getZoom() - 0.1); return; // zoom out
            case 'z': case 'Z': e.preventDefault(); applyZoom(1.2); return;             // zoom to 120%
            case 'G': e.preventDefault(); window.scrollTo(0, docBottom()); return; // Shift+G -> bottom
            case 'g': handleG(e); return;                                          // gg -> top
            case 't': case 'T': e.preventDefault(); toggleToc(); return;
            case 'r': case 'R':
                if (webview) {
                    e.preventDefault();
                    saveScroll();
                    webview.postMessage({type: 'refresh'});
                }
                return;
            case 'e': case 'E': case 'i': case 'I':
                if (webview) {
                    e.preventDefault();
                    webview.postMessage({type: 'edit'});
                }
                return;
        }
    });

    // Find-bar input: Enter = next, Shift+Enter = previous, Esc = close.
    // stopPropagation keeps these from reaching the document handler (whose
    // Escape would otherwise close the Lister).
    var findInput = document.getElementById('mdview-find-input');
    if (findInput) {
        findInput.addEventListener('keydown', function(e) {
            if (e.key === 'Enter') {
                e.preventDefault();
                e.stopPropagation();
                runFind(findInput.value, e.shiftKey);
            } else if (e.key === 'Escape') {
                e.preventDefault();
                e.stopPropagation();
                closeFind();
            } else {
                e.stopPropagation();
            }
        });
    }
})();
"#;

/// Markup for the TOC panel and its toggle button. Populated by `VIEWER_SCRIPT`;
/// hidden entirely when the document has no headings.
const TOC_MARKUP: &str = r#"<button id="mdview-toc-toggle" type="button" title="Toggle contents (t)" aria-label="Toggle table of contents">&#9776;</button>
<nav id="mdview-toc" aria-label="Table of contents">
<div id="mdview-toc-header">Contents</div>
<ul id="mdview-toc-list"></ul>
</nav>
"#;

/// Markup for the find bar, revealed by pressing `/`.
const FIND_MARKUP: &str = r#"<div id="mdview-find" role="search">
<input id="mdview-find-input" type="text" placeholder="Find…" aria-label="Find in page" autocomplete="off" spellcheck="false">
<span id="mdview-find-status" aria-live="polite"></span>
</div>
"#;

pub fn wrap_html(content: &str, dark_mode: bool) -> String {
    // (bg, text, code_bg, link, border, toc_bg, toc_hover, toc_active)
    let (
        bg_color,
        text_color,
        code_bg,
        link_color,
        border_color,
        toc_bg,
        toc_hover,
        toc_active,
        muted,
    ) = if dark_mode {
        (
            "#1e1e1e", "#d4d4d4", "#2d2d2d", "#58a6ff", "#444", "#252526", "#2d2d30", "#37373d",
            "#8b949e",
        )
    } else {
        (
            "#ffffff", "#24292e", "#f6f8fa", "#0366d6", "#e1e4e8", "#f6f8fa", "#eaeef2", "#dbe9ff",
            "#6a737d",
        )
    };

    let content = content
        .replace("src=\"/", "src=\"")
        .replace("href=\"/", "href=\"")
        .replace("src='/", "src='")
        .replace("href='/", "href='");

    let styles = STYLE_TEMPLATE
        .replace("%%BG%%", bg_color)
        .replace("%%TEXT%%", text_color)
        .replace("%%CODE_BG%%", code_bg)
        .replace("%%LINK%%", link_color)
        .replace("%%BORDER%%", border_color)
        .replace("%%TOC_BG%%", toc_bg)
        .replace("%%TOC_HOVER%%", toc_hover)
        .replace("%%TOC_ACTIVE%%", toc_active)
        .replace("%%MUTED%%", muted);

    let mut out = String::with_capacity(
        content.len() + styles.len() + VIEWER_SCRIPT.len() + TOC_MARKUP.len() + 256,
    );
    out.push_str("<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"UTF-8\">\n<style>");
    out.push_str(&styles);
    out.push_str("</style>\n</head>\n<body>\n");
    out.push_str(TOC_MARKUP);
    out.push_str(FIND_MARKUP);
    out.push_str(&content);
    out.push_str("\n<script>");
    out.push_str(VIEWER_SCRIPT);
    out.push_str("</script>\n</body>\n</html>");
    out
}

#[allow(dead_code)]
pub fn markdown_to_plain_text(markdown: &str) -> String {
    use pulldown_cmark::{Event, Tag, TagEnd};

    let options = Options::empty();
    let parser = Parser::new_ext(markdown, options);

    let mut output = String::new();

    for event in parser {
        match event {
            Event::Text(text) => output.push_str(&text),
            Event::Code(code) => {
                output.push('`');
                output.push_str(&code);
                output.push('`');
            }
            Event::SoftBreak | Event::HardBreak => output.push('\n'),
            Event::Start(Tag::Paragraph) => {}
            Event::End(TagEnd::Paragraph) => output.push_str("\n\n"),
            Event::Start(Tag::Heading { .. }) => {}
            Event::End(TagEnd::Heading(_)) => output.push_str("\n\n"),
            Event::Start(Tag::CodeBlock(_)) => output.push_str("\n```\n"),
            Event::End(TagEnd::CodeBlock) => output.push_str("```\n\n"),
            Event::Start(Tag::List(_)) => {}
            Event::End(TagEnd::List(_)) => output.push('\n'),
            Event::Start(Tag::Item) => output.push_str("  - "),
            Event::End(TagEnd::Item) => output.push('\n'),
            Event::Start(Tag::BlockQuote(_)) => output.push_str("> "),
            Event::End(TagEnd::BlockQuote(_)) => output.push('\n'),
            _ => {}
        }
    }

    output.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_markdown() {
        let md = "# Hello\n\nThis is **bold** and *italic*.";
        let html = markdown_to_html(md);
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
        assert!(html.contains("<em>italic</em>"));
    }

    #[test]
    fn test_code_block() {
        let md = "```rust\nfn main() {}\n```";
        let html = markdown_to_html(md);
        assert!(html.contains("<code"));
        assert!(html.contains("fn main()"));
    }

    #[test]
    fn test_preserve_relative_urls() {
        let md = "[Guide](../README.md#top)\n\n![Img](../assets/screenshot.png)\n\n[Top](#top)";
        let html = markdown_to_html(md);
        assert!(html.contains("href=\"../README.md#top\""));
        assert!(html.contains("src=\"../assets/screenshot.png\""));
        assert!(html.contains("href=\"#top\""));
    }

    #[test]
    fn test_wrap_html_includes_toc_scaffolding() {
        let html = wrap_html("<h1>Title</h1>", false);
        assert!(html.contains("id=\"mdview-toc\""));
        assert!(html.contains("id=\"mdview-toc-list\""));
        assert!(html.contains("id=\"mdview-toc-toggle\""));
        // Color placeholders must all be substituted.
        assert!(!html.contains("%%"));
    }

    #[test]
    fn test_frontmatter_rendered_as_comment_block() {
        let md = "---\ntitle: Hello\ndate: 2026-07-29\n---\n\n# Body\n";
        let html = markdown_to_html(md);
        assert!(html.contains("class=\"mdview-frontmatter\""));
        assert!(html.contains("title: Hello"));
        assert!(html.contains("<h1>Body</h1>"));
        let fm_pos = html.find("mdview-frontmatter").unwrap();
        let h1_pos = html.find("<h1>").unwrap();
        assert!(fm_pos < h1_pos, "front matter must precede the body");
        assert!(!html.contains("<hr"), "delimiters must not become <hr>");
    }

    #[test]
    fn test_frontmatter_closing_dots() {
        let md = "---\ntitle: X\n...\n\nbody\n";
        let html = markdown_to_html(md);
        assert!(html.contains("class=\"mdview-frontmatter\""));
        assert!(html.contains("title: X"));
    }

    #[test]
    fn test_frontmatter_html_escaped() {
        let md = "---\ntitle: <b>& stuff</b>\n---\n\nbody\n";
        let html = markdown_to_html(md);
        assert!(html.contains("&lt;b&gt;&amp; stuff"));
        assert!(!html.contains("<b>& stuff"));
    }

    #[test]
    fn test_lone_thematic_break_is_not_frontmatter() {
        // A `---` that is not a closed leading block stays a thematic break.
        let md = "intro\n\n---\n\nmore\n";
        let html = markdown_to_html(md);
        assert!(!html.contains("mdview-frontmatter"));
        assert!(html.contains("<hr"));
    }

    #[test]
    fn test_wrap_html_dark_mode_colors() {
        let dark = wrap_html("<h1>Title</h1>", true);
        assert!(dark.contains("#1e1e1e"));
        let light = wrap_html("<h1>Title</h1>", false);
        assert!(light.contains("#ffffff"));
    }
}
