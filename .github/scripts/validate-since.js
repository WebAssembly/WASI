#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

/**
 * Top-level declarations that require @since or @unstable annotations.
 * These are matched at the start of a line (with optional leading whitespace).
 */
const DECLARATION_PATTERNS = [
  { name: 'interface', regex: /^\s*interface\s+([a-z][a-z0-9-]*)\s*\{/i },
  { name: 'world', regex: /^\s*world\s+([a-z][a-z0-9-]*)\s*\{/i },
  { name: 'type', regex: /^\s*type\s+([a-z][a-z0-9-]*)\s*=/i },
  { name: 'record', regex: /^\s*record\s+([a-z][a-z0-9-]*)\s*\{/i },
  { name: 'variant', regex: /^\s*variant\s+([a-z][a-z0-9-]*)\s*\{/i },
  { name: 'enum', regex: /^\s*enum\s+([a-z][a-z0-9-]*)\s*\{/i },
  { name: 'flags', regex: /^\s*flags\s+([a-z][a-z0-9-]*)\s*\{/i },
  { name: 'resource', regex: /^\s*resource\s+([a-z][a-z0-9-]*)\s*[{;]/i },
];

/**
 * Annotation patterns that satisfy the @since requirement.
 */
const SINCE_PATTERN = /@since\s*\(\s*version\s*=\s*[0-9a-z.\-]+\s*\)/i;
const UNSTABLE_PATTERN = /@unstable\s*\(\s*feature\s*=\s*[a-z][a-z0-9-]*\s*\)/i;

/**
 * Check if a line has a preceding @since or @unstable annotation.
 * Looks backward through lines, skipping doc comments (///).
 */
function hasVersionAnnotation(lines, lineIndex, maxLookback = 20) {
  for (let i = 1; i <= Math.min(lineIndex, maxLookback); i++) {
    const prevLine = lines[lineIndex - i];
    if (!prevLine) continue;

    const trimmed = prevLine.trim();

    // Found @since annotation
    if (SINCE_PATTERN.test(trimmed)) {
      return true;
    }

    // Found @unstable annotation (accepted alternative)
    if (UNSTABLE_PATTERN.test(trimmed)) {
      return true;
    }

    // Skip doc comments - continue looking
    if (trimmed.startsWith('///')) {
      continue;
    }

    // Skip other annotations - continue looking
    if (trimmed.startsWith('@')) {
      continue;
    }

    // Skip empty lines - continue looking
    if (trimmed === '') {
      continue;
    }

    // Hit non-annotation, non-comment content - stop looking
    break;
  }

  return false;
}

/**
 * Validate a single WIT file for @since annotations.
 * @param {string} filePath - Path to the WIT file
 * @returns {Array} Array of error objects { file, line, declaration, name, message }
 */
function validateFile(filePath) {
  const errors = [];

  const content = fs.readFileSync(filePath, 'utf-8');
  const lines = content.split('\n');

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    for (const { name, regex } of DECLARATION_PATTERNS) {
      const match = line.match(regex);
      if (match) {
        if (!hasVersionAnnotation(lines, i)) {
          errors.push({
            file: filePath,
            line: i + 1, // 1-indexed for display
            declaration: name,
            name: match[1],
            message: `Missing @since annotation for ${name} '${match[1]}'`,
          });
        }
        break; // Only match one pattern per line
      }
    }
  }

  return errors;
}

/**
 * Validate all WIT files in a directory recursively.
 * Excludes deps/ directories.
 * @param {string} dirPath - Directory to validate
 * @returns {Array} Array of all errors
 */
function validateDirectory(dirPath) {
  const errors = [];

  function walkDir(dir) {
    const entries = fs.readdirSync(dir, { withFileTypes: true });
    for (const entry of entries) {
      const fullPath = path.join(dir, entry.name);

      if (entry.isDirectory()) {
        // Skip deps directories
        if (entry.name === 'deps') {
          continue;
        }
        walkDir(fullPath);
      } else if (entry.name.endsWith('.wit')) {
        errors.push(...validateFile(fullPath));
      }
    }
  }

  walkDir(dirPath);
  return errors;
}

/**
 * Format errors for GitHub Actions output (clickable annotations).
 * @param {Array} errors - Array of error objects
 * @returns {string} Formatted error output
 */
function formatErrors(errors) {
  return errors.map(err => {
    const relPath = path.relative(process.cwd(), err.file);
    return `::error file=${relPath},line=${err.line}::${err.message}`;
  }).join('\n');
}

// CLI usage: node validate-since.js <directory>
if (require.main === module) {
  const args = process.argv.slice(2);

  if (args.length === 0) {
    console.log('Usage: node validate-since.js <directory>');
    console.log('Example: node validate-since.js proposals/io/wit');
    process.exit(1);
  }

  const targetDir = args[0];

  if (!fs.existsSync(targetDir)) {
    console.error(`Directory not found: ${targetDir}`);
    process.exit(1);
  }

  console.log(`Validating @since annotations in ${targetDir}...\n`);

  const errors = validateDirectory(targetDir);

  if (errors.length > 0) {
    console.log(formatErrors(errors));
    console.log(`\n${errors.length} missing @since annotation(s) found.`);
    process.exit(1);
  } else {
    console.log('All declarations have @since annotations.');
    process.exit(0);
  }
}

module.exports = {
  validateFile,
  validateDirectory,
  formatErrors,
  DECLARATION_PATTERNS,
};
