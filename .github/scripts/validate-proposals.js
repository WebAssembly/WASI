#!/usr/bin/env node

const { execSync } = require('child_process');
const fs = require('fs');
const { validateDirectory, formatErrors } = require('./validate-since');

const witPath = (proposal, version) => {
  if (version === '0.2') return `proposals/${proposal}/wit`;
  if (version === '0.3') return `proposals/${proposal}/wit-0.3.0-draft`;
  throw new Error(`Unknown version: ${version}`);
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
const toValidate = [];
const filesByVersion = [
  [process.env.WIT_02_FILES, '0.2'],
  [process.env.WIT_03_FILES, '0.3'],
];

for (const [filesJson, version] of filesByVersion) {
  for (const proposal of extractProposals(parseFiles(filesJson))) {
    toValidate.push({ proposal, version });
  }
}

if (toValidate.length === 0) {
  console.log('No proposals to validate');
  process.exit(0);
}

let failed = false;

for (const { proposal, version } of toValidate) {
  const witDir = witPath(proposal, version);
  console.log(`::group::Validating ${proposal} v${version}`);

  try {
    console.log(`  Path: ${witDir}`);

    // Check wit-deps lock if deps.toml exists
    if (fs.existsSync(`${witDir}/deps.toml`)) {
      console.log('  Checking dependencies...');
      if (!run(`wit-deps -m "${witDir}"/deps.toml -l "${witDir}"/deps.lock -d "${witDir}"/deps lock --check`)) {
        console.log(`::error::wit-deps lock check failed for ${proposal} v${version}`);
        failed = true;
      }
    }

    // Validate WIT syntax
    console.log('  Validating WIT...');
    if (!run(`wasm-tools component wit "${witDir}" -o /dev/null`)) {
      console.log(`::error::WIT validation failed for ${proposal} v${version}`);
      failed = true;
    }

    // Validate wasm encoding
    console.log('  Validating wasm encoding...');
    if (!run(`wasm-tools component wit "${witDir}" --wasm -o /dev/null`)) {
      console.log(`::error::wasm encoding failed for ${proposal} v${version}`);
      failed = true;
    }

    // Validate @since annotations
    console.log('  Validating @since annotations...');
    const sinceErrors = validateDirectory(witDir);
    if (sinceErrors.length > 0) {
      console.log(formatErrors(sinceErrors));
      console.log(`::error::@since validation failed for ${proposal} v${version}: ${sinceErrors.length} missing annotation(s)`);
      failed = true;
    }
  } finally {
    console.log('::endgroup::');
  }
}

if (failed) {
  console.log('\n❌ Validation failed');
  process.exit(1);
} else {
  console.log('\n✅ All proposals validated successfully');
  process.exit(0);
}
