#!/usr/bin/env node

const { execSync } = require('child_process');
const fs = require('fs');
const os = require('os');
const path = require('path');
const { validateDirectory, formatErrors } = require('./validate-since');
const { readRecord, UNGATED } = require('./cm-features');

// `wasm-tools` feature name for each Component Model gate WASI has adopted.
// Adding a row to docs/ComponentModelFeatures.md without an entry here is a
// hard error rather than a silently un-enforced feature.
const GATE_FEATURES = {
  '🔀': 'cm-async',
  '🗺️': 'cm-map',
  '🏷️': 'cm-implements',
};

// Emoji gates carry a variation selector (U+FE0F) inconsistently across
// sources, so compare them without it.
const bare = (gate) => gate.replace(/\uFE0F/g, '');

// The adopted Component Model features, as `wasm-tools` CLI flags. Enabling
// them is what keeps proposals from depending on a feature the Subgroup has
// not voted to adopt: an un-adopted gate stays off and fails validation. Note
// these are added to whatever `wasm-tools` enables by default, so a gate that
// is un-adopted here but on by default there is not caught.
const adoptedFeatureFlags = () => {
  const lookup = new Map(Object.entries(GATE_FEATURES).map(([g, f]) => [bare(g), f]));
  const features = new Set();
  for (const { gate, version } of readRecord()) {
    if (gate === UNGATED) continue;
    const feature = lookup.get(bare(gate));
    if (!feature) {
      throw new Error(
        `docs/ComponentModelFeatures.md adopts '${gate}' for ${version}, but ` +
          `GATE_FEATURES in ${path.basename(__filename)} has no wasm-tools feature name for it`
      );
    }
    features.add(feature);
  }
  return [...features].map((feature) => `-f ${feature}`).join(' ');
};

const parseFiles = (filesJson) => {
  if (!filesJson || filesJson === 'null') return [];
  try {
    return JSON.parse(filesJson);
  } catch {
    return [];
  }
};

const extractProposals = (files) => {
  const proposals = new Set();
  for (const f of files) {
    const match = f.match(/^proposals\/([^/]+)\//);
    if (match) proposals.add(match[1]);
  }
  return [...proposals].sort();
};

const run = (cmd) => {
  console.log(`  $ ${cmd}`);
  try {
    const output = execSync(cmd, { encoding: 'utf-8', stdio: ['pipe', 'pipe', 'pipe'] });
    if (output.trim()) {
      console.log(output);
    }
    return true;
  } catch (err) {
    if (err.stdout) console.log(err.stdout);
    if (err.stderr) console.error(err.stderr);
    console.error(`  Exit code: ${err.status}`);
    return false;
  }
};

// Collect proposals to validate from changed files
const toValidate = extractProposals(parseFiles(process.env.WIT_FILES));

if (toValidate.length === 0) {
  console.log('No proposals to validate');
  process.exit(0);
}

let failed = false;

const featureFlags = adoptedFeatureFlags();
const outDir = fs.mkdtempSync(path.join(os.tmpdir(), 'wasi-validate-'));
console.log(`Component Model features: ${featureFlags || '(none adopted)'}`);

for (const proposal of toValidate) {
  const witDir = ((proposal) => `proposals/${proposal}/wit`)(proposal);
  console.log(`::group::Validating ${proposal}`);

  try {
    console.log(`  Path: ${witDir}`);

    // Skip proposals whose wit/ directory no longer exists. A PR that removes a
    // proposal still surfaces its deleted files in the changed-file list, but
    // there is nothing left to validate.
    if (!fs.existsSync(witDir)) {
      console.log(`  Skipping ${proposal}: ${witDir} no longer exists (proposal removed)`);
      continue;
    }

    // Check wit-deps lock if deps.toml exists
    if (fs.existsSync(`${witDir}/deps.toml`)) {
      console.log('  Checking dependencies...');
      if (!run(`wit-deps -m "${witDir}"/deps.toml -l "${witDir}"/deps.lock -d "${witDir}"/deps lock --check`)) {
        console.log(`::error::wit-deps lock check failed for ${proposal}`);
        failed = true;
      }
    }

    // Validate WIT syntax
    console.log('  Validating WIT...');
    if (!run(`wasm-tools component wit "${witDir}" -o /dev/null`)) {
      console.log(`::error::WIT validation failed for ${proposal}`);
      failed = true;
    }

    // Validate wasm encoding
    console.log('  Validating wasm encoding...');
    const encoded = path.join(outDir, `${proposal}.wasm`);
    if (!run(`wasm-tools component wit "${witDir}" --wasm -o "${encoded}"`)) {
      console.log(`::error::wasm encoding failed for ${proposal}`);
      failed = true;
    } else if (!run(`wasm-tools validate ${featureFlags} "${encoded}"`)) {
      // `component wit --wasm` does not enforce Component Model feature gates,
      // so the encoded package is validated separately against the features
      // WASI has adopted.
      console.log(`::error::Component Model validation failed for ${proposal}`);
      failed = true;
    }

    // Validate @since annotations
    console.log('  Validating @since annotations...');
    const sinceErrors = validateDirectory(witDir);
    if (sinceErrors.length > 0) {
      console.log(formatErrors(sinceErrors));
      console.log(`::error::@since validation failed for ${proposal}: ${sinceErrors.length} missing annotation(s)`);
      failed = true;
    }
  } finally {
    console.log('::endgroup::');
  }
}

fs.rmSync(outDir, { recursive: true, force: true });

if (failed) {
  console.log('\n❌ Validation failed');
  process.exit(1);
} else {
  console.log('\n✅ All proposals validated successfully');
  process.exit(0);
}
