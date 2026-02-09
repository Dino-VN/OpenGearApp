# release.ps1
param (
    [string]$Version = "patch" # major, minor, or patch
)

# 1. Update version in package.json and tauri.conf.json
Write-Host "Bumping version ($Version)..."
npm version $Version --no-git-tag-version

# Read new version from package.json
$packageJson = Get-Content "package.json" | ConvertFrom-Json
$newVersion = $packageJson.version
Write-Host "New version: $newVersion"

# Update tauri.conf.json version
$tauriConfPath = "src-tauri/tauri.conf.json"
$tauriJson = Get-Content $tauriConfPath -Raw | ConvertFrom-Json
$tauriJson.version = $newVersion
$tauriJson | ConvertTo-Json -Depth 100 | Set-Content $tauriConfPath

# 2. Git operations
Write-Host "Committing and tagging..."
git add package.json src-tauri/tauri.conf.json
git commit -m "chore(release): v$newVersion"
git tag "v$newVersion"

# 3. Push to trigger GitHub Action
Write-Host "Pushing to GitHub..."
git push origin main
git push origin "v$newVersion"

Write-Host "Release v$newVersion triggered! check GitHub Actions for progress."
