#!/usr/bin/env node

// Promote feature gates marked for stabilization during a release.
//
// Convention:
//
//     // @stabilize-next
//     @unstable(feature = cli-exit-with-code)
//     exit-with-code: func(status-code: u8);
//
// The `// @stabilize-next` marker line must sit immediately above an
// `@unstable(feature = ...)` gate. When a release for version NEXT is cut,
// this script rewrites the gate to `@since(version = NEXT)` and removes the
// marker line, so the feature graduates exactly when the package version is
// bumped to NEXT (the only point at which `@since(NEXT)` is legal).
//
// Usage: node stabilize-features.js <wit-dir> <next-version>

const fs = require('fs');
const path = require('path');

const MARKER_PATTERN = /^\s*\/\/\s*@stabilize-next\s*$/;
const UNSTABLE_PATTERN = /^(\s*)@unstable\s*\(\s*feature\s*=\s*([a-z][a-z0-9-]*)\s*\)\s*$/i;

function stabilizeFile(filePath, nextVersion) {
  const lines = fs.readFileSync(filePath, 'utf-8').split('\n');
  const out = [];
  const promoted = [];

  for (let i = 0; i < lines.length; i++) {
    if (MARKER_PATTERN.test(lines[i])) {
      const next = lines[i + 1];
      const m = next && next.match(UNSTABLE_PATTERN);
      if (!m) {
        throw new Error(
          `${filePath}:${i + 1}: '// @stabilize-next' is not immediately followed by an @unstable(...) gate`
        );
      }
      const indent = m[1];
      // Drop the marker line, replace the gate on the following line.
      out.push(`${indent}@since(version = ${nextVersion})`);
      promoted.push({ feature: m[2], line: i + 1 });
      i++; // consume the @unstable line we just rewrote
      continue;
    }
    out.push(lines[i]);
  }

  if (promoted.length > 0) {
    fs.writeFileSync(filePath, out.join('\n'));
  }
  return promoted;
}

function walkWit(dir, cb) {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      if (entry.name === 'deps') continue; // pulled packages, leave untouched
      walkWit(full, cb);
    } else if (entry.name.endsWith('.wit')) {
      cb(full);
    }
  }
}

function stabilizeDirectory(dir, nextVersion) {
  const all = [];
  walkWit(dir, (file) => {
    for (const p of stabilizeFile(file, nextVersion)) {
      all.push({ file, ...p });
    }
  });
  return all;
}

if (require.main === module) {
  const [dir, nextVersion] = process.argv.slice(2);
  if (!dir || !nextVersion) {
    console.error('Usage: node stabilize-features.js <wit-dir> <next-version>');
    process.exit(1);
  }
  if (!fs.existsSync(dir)) {
    console.error(`Directory not found: ${dir}`);
    process.exit(1);
  }

  try {
    const promoted = stabilizeDirectory(dir, nextVersion);
    if (promoted.length === 0) {
      console.log(`No @stabilize-next markers in ${dir}`);
    } else {
      for (const p of promoted) {
        const rel = path.relative(process.cwd(), p.file);
        console.log(`Stabilized feature '${p.feature}' -> @since(version = ${nextVersion}) (${rel})`);
      }
    }
  } catch (err) {
    console.error(`::error::${err.message}`);
    process.exit(1);
  }
}

module.exports = { stabilizeFile, stabilizeDirectory };
