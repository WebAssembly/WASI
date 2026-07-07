#!/usr/bin/env node

const { execSync } = require('child_process');
const fs = require('fs');
const { validateDirectory, formatErrors } = require('./validate-since');

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
    if (!run(`wasm-tools component wit "${witDir}" --wasm -o /dev/null`)) {
      console.log(`::error::wasm encoding failed for ${proposal}`);
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

if (failed) {
  console.log('\n❌ Validation failed');
  process.exit(1);
} else {
  console.log('\n✅ All proposals validated successfully');
  process.exit(0);
}
