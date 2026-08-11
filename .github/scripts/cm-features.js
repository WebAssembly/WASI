#!/usr/bin/env node

// Emit the "Component Model features" section of a specification overview.
//
// The adoption record in docs/ComponentModelFeatures.md is the source of
// truth: it lists every Component Model gated feature the WASI Subgroup has
// voted to adopt, and the release it was adopted for. Adoption is cumulative,
// so the section for a given version lists every feature adopted in that
// version or an earlier one.
//
// Prerelease versions (`0.3.0-rc-*`) are treated as their base version, since
// a release candidate carries the same feature baseline as the release it is
// a candidate for.
//
// Usage: node cm-features.js <version> [record-path]

const fs = require('fs');
const path = require('path');

const RECORD = path.join(__dirname, '..', '..', 'docs', 'ComponentModelFeatures.md');
const HEADING = '## Adopted features';

// The record marks the default (ungated) features with an em dash in the gate
// column. They are not a gated feature, so they are described in prose below
// rather than listed as a row.
const UNGATED = '—';

function parseVersion(version) {
  const base = version.split('-')[0];
  const parts = base.split('.').map((part) => Number(part));
  if (parts.length !== 3 || parts.some((part) => !Number.isInteger(part) || part < 0)) {
    throw new Error(`not a version: '${version}'`);
  }
  return parts;
}

function compareVersions(a, b) {
  const left = parseVersion(a);
  const right = parseVersion(b);
  for (let i = 0; i < 3; i++) {
    if (left[i] !== right[i]) return left[i] - right[i];
  }
  return 0;
}

// Rows of the adoption table, in document order, as {version, gate, feature}.
function readRecord(recordPath = RECORD) {
  const lines = fs.readFileSync(recordPath, 'utf-8').split('\n');
  const start = lines.indexOf(HEADING);
  if (start === -1) {
    throw new Error(`${recordPath}: no '${HEADING}' heading`);
  }

  const rows = [];
  for (let i = start + 1; i < lines.length; i++) {
    const line = lines[i].trim();
    if (line.startsWith('## ')) break;
    if (!line.startsWith('|')) continue;
    // Cells are read positionally, so a row missing its trailing pipe would
    // silently lose the last character of its final cell.
    if (!line.endsWith('|')) {
      throw new Error(`${recordPath}:${i + 1}: table row does not end with '|'`);
    }

    const cells = line.slice(1, -1).split('|').map((cell) => cell.trim());
    if (cells.length !== 4) {
      throw new Error(`${recordPath}:${i + 1}: expected 4 columns, found ${cells.length}`);
    }
    // Skip the header and its separator, in any of the alignment styles.
    if (cells[0] === 'WASI version') continue;
    if (cells.every((cell) => /^:?-+:?$/.test(cell))) continue;

    rows.push({ version: cells[0], gate: cells[1], feature: cells[2] });
  }

  if (rows.length === 0) {
    throw new Error(`${recordPath}: no rows under '${HEADING}'`);
  }
  return rows;
}

// The section is appended to a partially written overview, ahead of that
// file's link definitions, so it both starts and ends with a blank line: a
// link definition on the line directly below a paragraph would be parsed as
// part of that paragraph.
function render(version, rows) {
  const adopted = rows
    .filter((row) => row.gate !== UNGATED)
    .filter((row) => compareVersions(row.version, version) <= 0);

  const out = ['', '## Component Model features', ''];

  if (adopted.length === 0) {
    out.push(
      'Implementing this version of the specification requires the default (ungated)',
      '[Component Model][cm] features. No [gated features][gates] have been adopted.',
      '',
      ''
    );
    return out.join('\n');
  }

  out.push(
    'Implementing this version of the specification requires the default (ungated)',
    '[Component Model][cm] features, plus the [gated features][gates] adopted by the',
    'WASI Subgroup up to and including this version:',
    '',
    '| Gate | Feature | Adopted in |',
    '| --- | --- | --- |'
  );
  for (const row of adopted) {
    out.push(`| ${row.gate} | ${row.feature} | ${row.version} |`);
  }
  out.push(
    '',
    'These are required whether or not an API in this version uses them. See',
    '[Component Model features in WASI][features] for the full adoption record.',
    '',
    ''
  );
  return out.join('\n');
}

function main() {
  const [version, recordPath = RECORD] = process.argv.slice(2);
  if (!version) {
    console.error('Usage: node cm-features.js <version> [record-path]');
    process.exit(1);
  }
  process.stdout.write(render(version, readRecord(recordPath)));
}

if (require.main === module) {
  main();
}

module.exports = { RECORD, UNGATED, readRecord, render, compareVersions };
