#!/bin/bash

# Release script for WASI 0.3.x releases
#
# This script automates the release process:
# 1. Triggers the release.yml workflow to update versions and create PR
# 2. Waits for PR to be filed and CI to pass
# 3. Waits for manual PR review and merge
# 4. Creates a GitHub release to trigger publishing
# 5. Waits for publish workflow to complete (validates packages in CI)
#
# Usage:
#   ./release.sh --prev 0.3.0 --next 0.3.1

set -e
set -x

# Parse arguments
PREV_VERSION=""
NEXT_VERSION=""

while [[ $# -gt 0 ]]; do
  case $1 in
    --prev) PREV_VERSION="$2"; shift 2 ;;
    --next) NEXT_VERSION="$2"; shift 2 ;;
    -h|--help)
      echo "Usage:"
      echo "  $0 --prev <prev_version> --next <next_version>"
      echo ""
      echo "Example:"
      echo "  $0 --prev 0.3.0 --next 0.3.1"
      exit 0
      ;;
    *)
      echo "Unknown option: $1"
      echo "Use --help for usage information"
      exit 1
      ;;
  esac
done

# Configuration
DATE="$(date +'%Y-%m-%d')"
REPO="WebAssembly/WASI"

if [ -z "$PREV_VERSION" ] || [ -z "$NEXT_VERSION" ]; then
  echo "Error: release requires --prev and --next"
  echo "Example: $0 --prev 0.3.0 --next 0.3.1"
  exit 1
fi
TAG="v$NEXT_VERSION"

echo "============================================"
echo "WASI Release"
echo "============================================"
echo "Previous version: $PREV_VERSION"
echo "Next version: $NEXT_VERSION"
echo "Tag: $TAG"
echo "Repository: $REPO"
echo "============================================"

# Ensure we're operating on the correct repo
gh repo set-default "$REPO"

# Check if release already exists
if gh release view "$TAG" &>/dev/null; then
  echo "Error: Release $TAG already exists!"
  echo "If you need to re-run, delete the release first:"
  echo "  gh release delete $TAG --yes"
  exit 1
fi

# Step 1: Trigger the release workflow
echo ""
echo "Step 1: Triggering release.yml workflow..."

gh workflow run "release.yml" \
  -f prev_version="$PREV_VERSION" \
  -f next_version="$NEXT_VERSION"

# Wait for workflow to start
echo "Waiting for workflow to start..."
sleep 10

# Get the run ID
RUN_ID="$(gh run list --workflow "release.yml" --created "$DATE" --json databaseId --limit 1 | jq -r '.[0].databaseId')"
if [ -z "$RUN_ID" ] || [ "$RUN_ID" == "null" ]; then
  echo "Error: Could not find workflow run"
  exit 1
fi

echo "Workflow run ID: $RUN_ID"
echo "Waiting for workflow to complete..."
gh run watch "$RUN_ID" --exit-status || {
  echo "Error: Workflow failed!"
  gh run view "$RUN_ID" --log-failed
  exit 1
}

# Step 2: Wait for PR and CI
echo ""
echo "Step 2: Waiting for PR..."
sleep 5

PR_NUMBER="$(gh pr list --head "release-v$NEXT_VERSION" --json number --limit 1 | jq -r '.[0].number')"
if [ -z "$PR_NUMBER" ] || [ "$PR_NUMBER" == "null" ]; then
  echo "Error: Could not find PR for release-v$NEXT_VERSION"
  exit 1
fi

echo "PR #$PR_NUMBER created"

# Close and reopen to trigger CI (workaround for some CI configurations)
echo "Retriggering CI..."
gh pr close "$PR_NUMBER"
gh pr reopen "$PR_NUMBER"

echo "Waiting for CI checks to pass..."
sleep 10
gh pr checks "$PR_NUMBER" --watch || {
  echo "Warning: Some checks may have failed. Continuing anyway..."
}

# Step 3: Wait for manual PR review and merge
echo ""
echo "Step 3: PR ready for review"
echo "============================================"
echo "PR #$PR_NUMBER: https://github.com/$REPO/pull/$PR_NUMBER"
echo "============================================"
echo ""
read -r -p "Press Enter after the PR has been reviewed and merged..."

# Verify PR was actually merged
STATE="$(gh pr view "$PR_NUMBER" --json state --jq '.state')"
if [ "$STATE" != "MERGED" ]; then
  echo "Error: PR #$PR_NUMBER is not merged (state: $STATE)"
  exit 1
fi

# Step 4: Create GitHub release
echo ""
echo "Step 4: Creating GitHub release $TAG..."
sleep 5

gh release create "$TAG" --generate-notes
gh release view "$TAG"

# Step 5: Wait for publish workflow
echo ""
echo "Step 5: Waiting for publish workflow to complete..."
sleep 10

PUBLISH_RUN_ID="$(gh run list --workflow "publish.yml" --created "$DATE" --json databaseId --limit 1 | jq -r '.[0].databaseId')"
if [ -z "$PUBLISH_RUN_ID" ] || [ "$PUBLISH_RUN_ID" == "null" ]; then
  echo "Warning: Could not find publish workflow run. It may not have started yet."
  sleep 30
  PUBLISH_RUN_ID="$(gh run list --workflow "publish.yml" --created "$DATE" --json databaseId --limit 1 | jq -r '.[0].databaseId')"
fi

if [ -n "$PUBLISH_RUN_ID" ] && [ "$PUBLISH_RUN_ID" != "null" ]; then
  echo "Publish workflow run ID: $PUBLISH_RUN_ID"
  gh run watch "$PUBLISH_RUN_ID" --exit-status || {
    echo "Error: Publish workflow failed!"
    gh run view "$PUBLISH_RUN_ID" --log-failed
    exit 1
  }
fi

echo ""
echo "============================================"
echo "✓ Release $NEXT_VERSION completed successfully!"
echo "============================================"
