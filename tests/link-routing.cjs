// Exercise the actual generated-page click handler without requiring WebView2.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const source = fs.readFileSync('src/markdown.rs', 'utf8');
const start = source.indexOf("    document.addEventListener('click', function(e) {", source.indexOf("// ---- Boot ----"));
assert(start >= 0);
const end = source.indexOf('\n    });', start);
const handler = source.slice(start, end + '\n    });'.length);
function click(href, ctrlKey = false, hosted = true) {
    let callback;
    const messages = [], fragments = [];
    let prevented = false;
    const base = 'https://mdview.example/notes/current.md';
    new Function('document', 'window', 'location', 'scrollToHash', handler)(
        { addEventListener: (_, fn) => { callback = fn; } },
        hosted ? { chrome: { webview: { postMessage: msg => messages.push(msg) } } } : {},
        { href: base }, fragment => fragments.push(fragment)
    );
    callback({ ctrlKey, target: { closest: () => ({
        getAttribute: () => href, href: new URL(href, base).href
    }) }, preventDefault: () => { prevented = true; } });
    return { prevented, messages, fragments };
}
for (const href of ['report.docx', '../财务/最终 版本.PDF', 'sheet.xlsx', 'image.png',
                    'report.docx?version=2#page', 'file:///C:/Reports/report.docx']) {
    const result = click(href);
    assert.equal(result.prevented, true, href);
    assert.equal(result.messages[0].type, 'openLink', href);
    assert.equal(result.messages[0].url, new URL(href, 'https://mdview.example/notes/current.md').href);
}
for (const href of ['next.md', '../目录/第二篇.MARKDOWN#标题', 'next%2Emd',
                    'https://example.com/report.docx', 'mailto:someone@example.com']) {
    const result = click(href);
    assert.equal(result.prevented, false, href);
    assert.deepEqual(result.messages, [], href);
}
assert.deepEqual(click('#heading').fragments, ['#heading']);
assert.equal(click('next.md', true).messages[0].type, 'openLink');
assert.equal(click('report.docx', false, false).prevented, false);
console.log('Link routing passed: local attachments, Markdown, fragments, external URLs, Ctrl+click, and exported HTML.');
