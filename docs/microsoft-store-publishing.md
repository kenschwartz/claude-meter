# Publishing ZaiMeter to the Microsoft Store

A comprehensive guide for packaging and publishing ZaiMeter (a Rust-based Windows system tray
application) to the Microsoft Store, with an alternative section on winget distribution.

**Last updated:** March 2025

---

## Table of Contents

1. [Overview of Distribution Options](#1-overview-of-distribution-options)
2. [Developer Account (Microsoft Partner Center)](#2-developer-account-microsoft-partner-center)
3. [MSIX Packaging](#3-msix-packaging)
4. [Code Signing](#4-code-signing)
5. [Store Listing Assets](#5-store-listing-assets)
6. [Submission Process in Partner Center](#6-submission-process-in-partner-center)
7. [Certification Requirements and Common Rejections](#7-certification-requirements-and-common-rejections)
8. [System Tray App Considerations](#8-system-tray-app-considerations)
9. [Auto-Updates: Store vs App-Managed](#9-auto-updates-store-vs-app-managed)
10. [Rust/Cargo-Specific Tools](#10-rustcargo-specific-tools)
11. [Alternative: Publishing to winget](#11-alternative-publishing-to-winget)
12. [Alternative: Unpackaged EXE Submission](#12-alternative-unpackaged-exe-submission)
13. [Recommended Strategy for ZaiMeter](#13-recommended-strategy-for-zaimeter)

---

## 1. Overview of Distribution Options

There are three main ways to distribute ZaiMeter through Microsoft's ecosystem:

| Method | Packaging | Signing | Store Listing | Auto-Update |
|--------|-----------|---------|---------------|-------------|
| **MSIX (Store)** | Full MSIX package | Microsoft re-signs | Full Store page | Store-managed |
| **Unpackaged EXE (Store)** | Just link to installer URL | Your own signing | Full Store page | You host the installer |
| **winget** | None (bare .exe/.msi) | Optional | CLI only (`winget install`) | Manifest update via PR |

For a small, portable .exe like ZaiMeter, all three are viable. They can also be combined:
publish to the Store for discoverability while also maintaining a winget manifest for CLI users.

---

## 2. Developer Account (Microsoft Partner Center)

### Registration

1. Go to https://developer.microsoft.com/en-us/microsoft-store/register
2. Sign in with a Microsoft account (personal or organizational).
3. Choose account type:
   - **Individual** -- Free (as of September 2025, Microsoft removed the registration fee for
     individual developers).
   - **Company** -- One-time $99 USD fee. Requires business verification (D-U-N-S number, legal
     entity information).

### What You Need

- A Microsoft account (Outlook/Hotmail/Live or any email linked to a Microsoft account).
- For company accounts: legal business name, address, D-U-N-S number, a business email.
- Tax profile and payout account (required if you plan to charge for the app; optional for free
  apps, but still recommended to set up).

### Timeline

- Individual accounts are typically approved within minutes to a few hours.
- Company accounts require business verification, which can take 1-5 business days.

### References

- [Open a developer account](https://learn.microsoft.com/en-us/windows/apps/publish/partner-center/open-a-developer-account)
- [Account types, locations, and fees](https://learn.microsoft.com/en-us/windows/apps/publish/partner-center/account-types-locations-and-fees)
- [Free registration for individual developers](https://learn.microsoft.com/en-us/windows/apps/publish/whats-new-individual-developer)

---

## 3. MSIX Packaging

MSIX is Microsoft's modern application packaging format. It wraps your .exe in a container that
provides clean install/uninstall, sandboxing, and Store compatibility.

### 3.1 What You Need

- **Windows 10 SDK** (includes `makeappx.exe` and `signtool.exe`)
- **Your built `zaimeter.exe`** (release build, ~3 MB)
- **An `AppxManifest.xml`** file describing the package
- **Visual assets** (icons at various sizes)
- Optionally: **MSIX Packaging Tool** (free from Microsoft Store) for GUI-based packaging

### 3.2 AppxManifest.xml

Create an `AppxManifest.xml` in a staging directory alongside your .exe. Here is a template
tailored for ZaiMeter:

```xml
<?xml version="1.0" encoding="utf-8"?>
<Package
  xmlns="http://schemas.microsoft.com/appx/manifest/foundation/windows10"
  xmlns:uap="http://schemas.microsoft.com/appx/manifest/uap/windows10"
  xmlns:rescap="http://schemas.microsoft.com/appx/manifest/foundation/windows10/restrictedcapabilities"
  xmlns:desktop="http://schemas.microsoft.com/appx/manifest/desktop/windows10"
  xmlns:uap10="http://schemas.microsoft.com/appx/manifest/uap/windows10/10"
  IgnorableNamespaces="uap rescap desktop uap10">

  <Identity
    Name="YourPublisherId.ZaiMeter"
    Publisher="CN=Your Publisher Name"
    Version="1.10.3.0"
    ProcessorArchitecture="x64" />

  <Properties>
    <DisplayName>ZaiMeter</DisplayName>
    <PublisherDisplayName>klivak</PublisherDisplayName>
    <Logo>Assets\StoreLogo.png</Logo>
    <Description>Monitor your Claude AI subscription usage in real-time from the system tray.</Description>
  </Properties>

  <Dependencies>
    <TargetDeviceFamily
      Name="Windows.Desktop"
      MinVersion="10.0.17763.0"
      MaxVersionTested="10.0.26100.0" />
  </Dependencies>

  <Resources>
    <Resource Language="en-us" />
  </Resources>

  <Applications>
    <Application
      Id="ZaiMeter"
      Executable="zaimeter.exe"
      EntryPoint="Windows.FullTrustApplication">

      <uap:VisualElements
        DisplayName="ZaiMeter"
        Description="Claude AI usage monitor for Windows"
        BackgroundColor="transparent"
        Square150x150Logo="Assets\Square150x150Logo.png"
        Square44x44Logo="Assets\Square44x44Logo.png" />

      <!-- Auto-start at login (replaces registry-based autostart) -->
      <Extensions>
        <desktop:Extension Category="windows.startupTask" Executable="zaimeter.exe"
                           EntryPoint="Windows.FullTrustApplication">
          <desktop:StartupTask TaskId="ZaiMeterStartup" Enabled="true"
                               DisplayName="ZaiMeter" />
        </desktop:Extension>
      </Extensions>

    </Application>
  </Applications>

  <Capabilities>
    <Capability Name="internetClient" />
    <rescap:Capability Name="runFullTrust" />
  </Capabilities>

</Package>
```

**Important notes:**
- `Identity.Name` and `Identity.Publisher` will be assigned by Partner Center when you reserve
  your app name. Replace the placeholder values with the real ones.
- `Version` must be in `Major.Minor.Build.Revision` format (4 parts).
- `EntryPoint="Windows.FullTrustApplication"` is required for Win32 desktop apps.
- `runFullTrust` capability is required for unpackaged Win32 apps running inside MSIX.
- The `windows.startupTask` extension replaces ZaiMeter's current registry-based autostart
  mechanism when running as an MSIX package.

### 3.3 Directory Layout

Create a staging directory with this structure:

```
msix-staging/
  AppxManifest.xml
  zaimeter.exe
  Assets/
    StoreLogo.png           (50x50)
    Square44x44Logo.png     (44x44)
    Square150x150Logo.png   (150x150)
    Wide310x150Logo.png     (310x150, optional)
    LargeTile.png           (310x310, optional)
```

### 3.4 Building the MSIX Package

Using `makeappx.exe` from the Windows SDK:

```powershell
# Find makeappx.exe (typically in Windows SDK bin directory)
# Example path:
$makeappx = "C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64\makeappx.exe"

# Create the MSIX package
& $makeappx pack /d ".\msix-staging" /p "ZaiMeter.msix" /v /h SHA256
```

Or if you prefer to use a mapping file:

```powershell
& $makeappx pack /f "mapping.txt" /p "ZaiMeter.msix" /v /h SHA256
```

### 3.5 MSIX Packaging Tool (GUI Alternative)

For a GUI-based approach:

1. Install "MSIX Packaging Tool" from the Microsoft Store (free).
2. Choose "Application package" > "Create your app package on this computer."
3. Point it to `zaimeter.exe` as the installer.
4. The tool will capture the installation and produce an MSIX package.

**Note:** Since ZaiMeter is a portable .exe (no installer), the MSIX Packaging Tool's
capture-based approach is less ideal. The manual `makeappx.exe` method described above is
more appropriate.

### References

- [Generating MSIX package components](https://learn.microsoft.com/en-us/windows/msix/desktop/desktop-to-uwp-manual-conversion)
- [Create an app package with MakeAppx.exe](https://learn.microsoft.com/en-us/windows/msix/package/create-app-package-with-makeappx-tool)
- [MSIX Packaging Tool overview](https://learn.microsoft.com/en-us/windows/msix/packaging-tool/tool-overview)

---

## 4. Code Signing

### 4.1 For Store Submission

When you submit an MSIX package to the Microsoft Store, **Microsoft re-signs the package** with
their own trusted certificate before distributing it to users. However, you still need to sign
the package yourself for upload validation.

For Store submission, you can use:
- A **self-signed certificate** (sufficient for upload; Microsoft re-signs before distribution).
- The publisher name in your certificate **must match** the `Identity.Publisher` field in your
  `AppxManifest.xml` exactly.

### 4.2 Creating a Self-Signed Certificate (for Testing and Store Upload)

```powershell
# Create a self-signed certificate
New-SelfSignedCertificate `
  -Type Custom `
  -Subject "CN=Your Publisher Name" `
  -KeyUsage DigitalSignature `
  -FriendlyName "ZaiMeter Dev Certificate" `
  -CertStoreLocation "Cert:\CurrentUser\My" `
  -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.3", "2.5.29.19={text}")

# Export to PFX (for use with signtool)
$cert = Get-ChildItem "Cert:\CurrentUser\My" | Where-Object { $_.Subject -eq "CN=Your Publisher Name" }
$password = ConvertTo-SecureString -String "YourPassword" -Force -AsPlainText
Export-PfxCertificate -Cert $cert -FilePath "ZaiMeter.pfx" -Password $password
```

### 4.3 Signing the MSIX Package

```powershell
$signtool = "C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64\signtool.exe"

# Sign with PFX
& $signtool sign /fd SHA256 /a /f "ZaiMeter.pfx" /p "YourPassword" "ZaiMeter.msix"
```

### 4.4 For Sideloading (Outside the Store)

If distributing MSIX outside the Store (e.g., via GitHub Releases), you need a certificate
trusted by the end user's machine. Options:

| Certificate Type | Cost | Trust Level |
|-----------------|------|-------------|
| Self-signed | Free | Must be manually installed on each device |
| Azure Trusted Signing | ~$10/month | Trusted by all Windows devices |
| Commercial code signing (DigiCert, Sectigo, etc.) | $200-500/year | Trusted by all Windows devices |

### 4.5 Important Rules

- The `CN=` value in your certificate **must exactly match** `Identity.Publisher` in
  `AppxManifest.xml`.
- Always use SHA-256 for signing (not SHA-1).
- Use timestamping when signing for production: `/tr http://timestamp.digicert.com /td SHA256`
- Never commit `.pfx` files or certificate passwords to source control.

### References

- [Create a certificate for package signing](https://learn.microsoft.com/en-us/windows/msix/package/create-certificate-package-signing)
- [Sign an app package](https://learn.microsoft.com/en-us/windows/msix/package/signing-package-overview)
- [MSIX and code signing certificates](https://www.advancedinstaller.com/msix-certificates-developer.html)

---

## 5. Store Listing Assets

### 5.1 Required Assets

| Asset | Dimensions | Format | Notes |
|-------|-----------|--------|-------|
| **Store logo** | 300x300 px | PNG | Used in search results and Store page |
| **Screenshot (at least 1)** | 1366x768 or 768x1366 | PNG | Desktop screenshots; 4-8 recommended |
| **App description** | Max 10,000 chars | Text | Short description (max 252 chars) also required |
| **Privacy policy URL** | -- | URL | Required if app accesses personal data or network |

### 5.2 Recommended Additional Assets

| Asset | Dimensions | Format | Notes |
|-------|-----------|--------|-------|
| Additional screenshots | 1366x768 | PNG | Up to 10 desktop screenshots |
| Hero image | 1920x1080 | PNG | Featured placement in Store |
| Square 44x44 icon | 44x44 | PNG | For MSIX manifest |
| Square 150x150 icon | 150x150 | PNG | For MSIX manifest |
| Wide 310x150 tile | 310x150 | PNG | Optional, for Start menu tile |
| Promotional trailer | -- | MP4 | Optional, up to 30 seconds |

### 5.3 ZaiMeter-Specific Listing Content

**Suggested short description:**
> Monitor your Claude AI subscription usage limits in real-time from the Windows system tray.

**Key features to highlight:**
- Real-time Claude usage monitoring (Opus, Sonnet, and all plan tiers)
- Ultra-lightweight: under 10 MB RAM, ~3 MB on disk
- System tray icon with tooltip showing current usage
- Dashboard popup with detailed usage breakdown and history
- Dark/Light/Auto theme support
- 10+ languages supported
- Auto-start with Windows
- No account needed -- reads existing Claude Code credentials

**Privacy policy:** Required because ZaiMeter accesses the Anthropic API (network access)
and reads credentials from Windows Credential Manager. Create a privacy policy page (can be
hosted on GitHub Pages or the project's GitHub repository) that covers:
- What data is accessed (OAuth token from local Credential Manager, API usage data)
- That no data is collected or transmitted to third parties
- That all data stays on the user's machine
- Contact information

**Age rating:** Complete the IARC questionnaire. ZaiMeter is a developer utility tool with no
objectionable content -- it will likely receive a "3+" or "Everyone" rating.

### References

- [Screenshots and images for MSIX apps](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msix/screenshots-and-images)
- [Microsoft Store policies](https://learn.microsoft.com/en-us/windows/apps/publish/store-policies)

---

## 6. Submission Process in Partner Center

### Step-by-Step

1. **Reserve app name**
   - Go to Partner Center > Apps and Games > New product > MSIX or PWA app.
   - Reserve "ZaiMeter" as your app name.
   - This also generates your `Identity.Name` and `Identity.Publisher` values for the manifest.

2. **Create submission**
   - Click "Start your submission" on the app overview page.
   - Fill in each section (described below).

3. **Pricing and availability**
   - Set price to "Free."
   - Choose markets (typically "All markets" or specific countries).
   - Set visibility: Public (or Private if you want a hidden link first for testing).

4. **Properties**
   - Category: Developer Tools > Utilities (or Productivity).
   - System requirements: Windows 10 version 1809 (build 17763) or later, x64 only.
   - Declare any restricted capabilities used.

5. **Age ratings**
   - Complete the IARC questionnaire (takes 2-5 minutes).

6. **Packages**
   - Upload your signed `.msix` file (or `.msixupload`).
   - Microsoft validates the package structure, manifest, and architecture.
   - The upload page shows detected capabilities, target platforms, and version.

7. **Store listings**
   - Fill in description, screenshots, search terms, and privacy policy URL.
   - Create listings for each language you support (or at least English).

8. **Notes for certification**
   - Explain that the app requires an existing Claude Code OAuth session to function.
   - Provide instructions on how to test (e.g., "Install Claude Code CLI and log in first").
   - This is crucial -- if testers cannot use the app, it may be rejected.

9. **Submit for certification**
   - Click "Submit to the Store."
   - Certification typically takes 1-3 business days (often within 24 hours).

### References

- [Create an app submission](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msix/create-app-submission)
- [Upload app packages](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msix/upload-app-packages)
- [Get started with Microsoft Store](https://learn.microsoft.com/en-us/windows/apps/publish/get-started)

---

## 7. Certification Requirements and Common Rejections

### What Microsoft Tests

- **Malware scan:** The binary is scanned for known malware signatures.
- **Technical compliance:** App must not crash on launch, must handle errors gracefully.
- **Policy compliance:** Content must match the description; no misleading claims.
- **Privacy:** If app accesses personal data or network, a privacy policy URL is required.
- **Functionality:** App must be functional and provide value as described.

### Common Rejection Reasons (and How to Avoid Them)

| Reason | Mitigation for ZaiMeter |
|--------|---------------------------|
| **App crashes or is not functional** | Test the MSIX package on a clean Windows install. Ensure the app works even without credentials (show a helpful message). |
| **Missing privacy policy** | Host a privacy policy (e.g., on GitHub Pages) and provide the URL. |
| **Misleading description** | Be accurate about what the app does and its requirements. |
| **Requires external dependencies not explained** | Clearly state in the description and certification notes that Claude Code CLI must be installed. |
| **Test account not provided** | Explain in "Notes for certification" how to test (or that testing requires a Claude subscription). |
| **App does nothing without sign-in** | Ensure the app shows a meaningful UI even without credentials (e.g., "No Claude Code session found. Please install Claude Code and log in."). |
| **Restricted capability misuse** | Only request `runFullTrust` and `internetClient`; both are standard for Win32 desktop apps. |

### Tips for First-Time Approval

1. Make sure the app launches and shows something useful even without valid credentials.
2. Include detailed testing instructions in the certification notes.
3. Use accurate, non-promotional language in the Store listing.
4. Test the MSIX package on a clean VM before submitting.

### References

- [Avoid common certification failures](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/avoid-common-certification-failures)
- [Desktop certification requirements](https://learn.microsoft.com/en-us/windows/win32/win_cert/certification-requirements-for-windows-desktop-apps)
- [Store policies](https://learn.microsoft.com/en-us/windows/apps/publish/store-policies)

---

## 8. System Tray App Considerations

ZaiMeter is a system tray application, which introduces several packaging-specific concerns.

### 8.1 Auto-Start Behavior

ZaiMeter currently uses the Windows registry (`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`)
for auto-start. Inside an MSIX container:

- **Registry writes to `HKCU\...\Run` are virtualized** and may not persist across updates.
- **Use the `windows.startupTask` extension** in the manifest instead (see Section 3.2).
- The startup task can be enabled/disabled by the user via Settings > Apps > Startup.
- You may need to conditionally detect whether running as MSIX and skip the registry-based
  autostart code. Check for package identity at runtime:

```rust
// Detect if running as MSIX packaged app
fn is_msix_packaged() -> bool {
    // Try to get the package family name
    // If this succeeds, we're running as a packaged app
    use windows::Win32::Storage::Packaging::Appx::GetCurrentPackageFullName;
    let mut length = 0u32;
    let result = unsafe { GetCurrentPackageFullName(&mut length, None) };
    // APPMODEL_ERROR_NO_PACKAGE (15700) means not packaged
    result.0 != 15700
}
```

### 8.2 File System Virtualization

MSIX packages run in a lightweight container with file system virtualization:

- **Config file location:** `std::env::current_exe()` parent directory will be inside the MSIX
  virtual file system, which is read-only. You need to use an app-writable location instead:
  - Use `%LOCALAPPDATA%\Packages\<PackageFamilyName>\LocalState\` for config.
  - Or detect MSIX and use `%LOCALAPPDATA%\ZaiMeter\` as a fallback.
- **SQLite database:** Same concern -- must be in a writable location.

### 8.3 Windows Credential Manager Access

ZaiMeter reads OAuth tokens from Windows Credential Manager. This works normally from inside
MSIX -- `CredReadW` calls are not affected by MSIX virtualization. No changes needed.

### 8.4 Single-Instance Mutex

Named mutexes (`CreateMutexW`) work normally inside MSIX. The existing
`"ZaiMeter-SingleInstance"` mutex will function correctly.

### 8.5 Toast Notifications

ZaiMeter uses PowerShell-based toast notifications. Inside MSIX:
- The app has a proper AppUserModelID (AUMID) assigned by the package.
- PowerShell-based notifications should still work, but consider using the Windows notification
  API directly (via the `windows` crate) for better integration with the notification center.
- Packaged apps with AUMID get proper grouping in Action Center.

### 8.6 Shell Extension Limitation

MSIX does **not** support in-process shell extensions. ZaiMeter does not use shell extensions,
so this is not an issue.

---

## 9. Auto-Updates: Store vs App-Managed

### 9.1 Microsoft Store Updates

When distributed through the Store:
- The Store checks for updates approximately **every 8 hours**.
- Updates are downloaded in the background.
- Updates are installed when the app is closed (non-disruptive).
- Users can also manually check for updates in the Store app.
- You publish a new version by creating a new submission with an updated MSIX package.

### 9.2 ZaiMeter's Built-In Updater

ZaiMeter has its own update checker (`src/updater.rs`) that polls GitHub Releases. When
distributed via the Store:

- **Disable the built-in updater** for Store builds. The Store should be the sole update channel
  to avoid conflicts and user confusion.
- Use a compile-time feature flag or runtime MSIX detection:

```rust
// In updater.rs, skip update check if running as Store app
if is_msix_packaged() {
    log::debug!("Running as Store app, skipping self-update check");
    return Ok(None);
}
```

### 9.3 Version Numbering

- Store versions use `Major.Minor.Build.Revision` (4-part).
- Each submission must have a higher version than the previous.
- Keep `Cargo.toml` version and MSIX version in sync (just append `.0` for the revision part).

### References

- [Auto-update and repair overview](https://learn.microsoft.com/en-us/windows/msix/app-installer/auto-update-and-repair--overview)
- [Store-published app updates from code](https://learn.microsoft.com/en-us/windows/msix/store-developer-package-update)

---

## 10. Rust/Cargo-Specific Tools

### 10.1 cargo-wix (MSI Installer)

[cargo-wix](https://github.com/volks73/cargo-wix) is a mature cargo subcommand for building
Windows `.msi` installers using the WiX Toolset.

```bash
cargo install cargo-wix
cargo wix init    # Generates wix/main.wxs template
cargo wix         # Builds the MSI installer
```

- Produces `.msi` files, not `.msix`.
- Supports code signing.
- Useful for winget distribution (winget accepts .msi) or the unpackaged Store submission path.
- Well-maintained, actively used in the Rust ecosystem.

### 10.2 cargo-msix (Experimental)

[cargo-msix](https://github.com/davidanthoff/cargo-msix) is a cargo subcommand specifically for
building MSIX packages.

- **Status: Work in progress / not functional** as of early 2025.
- Not recommended for production use at this time.
- Watch the repository for future updates.

### 10.3 msix crate (Library)

The [msix](https://crates.io/crates/msix) crate is a Rust library for creating and signing MSIX
packages programmatically. Could be used in a custom build script but requires manual integration.

### 10.4 Recommended Approach for ZaiMeter

Since cargo-msix is not ready, use the **manual approach**:

1. Build with `cargo build --release`.
2. Copy `zaimeter.exe` to a staging directory with `AppxManifest.xml` and assets.
3. Run `makeappx.exe pack` to create the `.msix`.
4. Run `signtool.exe sign` to sign it.
5. Automate steps 2-4 in a PowerShell script or GitHub Actions workflow.

Example automation script (`scripts/build-msix.ps1`):

```powershell
param(
    [string]$Version = "1.10.3.0",
    [string]$CertPath = "ZaiMeter.pfx",
    [string]$CertPassword
)

$ErrorActionPreference = "Stop"

# Paths
$sdkBin = "C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64"
$stagingDir = ".\msix-staging"
$outputMsix = ".\ZaiMeter-$Version.msix"

# Step 1: Build release
Write-Host "Building release..."
cargo build --release

# Step 2: Prepare staging directory
Write-Host "Preparing staging directory..."
if (Test-Path $stagingDir) { Remove-Item $stagingDir -Recurse -Force }
New-Item -ItemType Directory -Path $stagingDir | Out-Null
New-Item -ItemType Directory -Path "$stagingDir\Assets" | Out-Null

# Copy binary
Copy-Item "target\release\zaimeter.exe" "$stagingDir\"

# Copy manifest (update version in manifest first)
$manifest = Get-Content "msix\AppxManifest.xml" -Raw
$manifest = $manifest -replace 'Version="[\d.]+"', "Version=`"$Version`""
Set-Content "$stagingDir\AppxManifest.xml" $manifest

# Copy assets
Copy-Item "msix\Assets\*" "$stagingDir\Assets\"

# Step 3: Create MSIX package
Write-Host "Creating MSIX package..."
& "$sdkBin\makeappx.exe" pack /d $stagingDir /p $outputMsix /v /h SHA256
if ($LASTEXITCODE -ne 0) { throw "makeappx.exe failed" }

# Step 4: Sign the package
if ($CertPath -and (Test-Path $CertPath)) {
    Write-Host "Signing MSIX package..."
    & "$sdkBin\signtool.exe" sign /fd SHA256 /a /f $CertPath /p $CertPassword `
        /tr http://timestamp.digicert.com /td SHA256 $outputMsix
    if ($LASTEXITCODE -ne 0) { throw "signtool.exe failed" }
}

Write-Host "Done: $outputMsix"

# Cleanup
Remove-Item $stagingDir -Recurse -Force
```

---

## 11. Alternative: Publishing to winget

winget (Windows Package Manager) is a command-line package manager built into Windows 11 and
available for Windows 10. Publishing to winget provides discoverability for CLI-savvy users.

### 11.1 Manifest Format

winget uses YAML manifest files in the [microsoft/winget-pkgs](https://github.com/microsoft/winget-pkgs)
repository. For a single .exe, a singleton manifest is sufficient:

```yaml
# manifests/k/klivak/ZaiMeter/1.10.3/klivak.ZaiMeter.yaml
PackageIdentifier: klivak.ZaiMeter
PackageVersion: "1.10.3"
PackageName: ZaiMeter
Publisher: klivak
License: MIT
LicenseUrl: https://github.com/klivak/zaimeter/blob/main/LICENSE
ShortDescription: Monitor Claude AI subscription usage limits from the Windows system tray.
Description: |-
  ZaiMeter is an ultra-lightweight Windows system tray application that monitors
  Claude AI subscription usage limits in real-time. Under 10 MB RAM, single portable .exe.
PackageUrl: https://github.com/klivak/zaimeter
Tags:
  - claude
  - ai
  - monitor
  - system-tray
  - usage
  - developer-tools
Installers:
  - Architecture: x64
    InstallerUrl: https://github.com/klivak/zaimeter/releases/download/v1.10.3/zaimeter.exe
    InstallerSha256: <SHA256_HASH_OF_EXE>
    InstallerType: portable
    Commands:
      - zaimeter
ManifestType: singleton
ManifestVersion: 1.6.0
```

### 11.2 Submission Process

1. **Fork** https://github.com/microsoft/winget-pkgs
2. **Create the manifest** file(s) in the correct path:
   `manifests/k/klivak/ZaiMeter/1.10.3/`
3. **Validate** locally:
   ```bash
   winget validate manifests/k/klivak/ZaiMeter/1.10.3/
   ```
4. **Submit a pull request** to the winget-pkgs repository.
5. Automated bots validate the manifest and test the installer.
6. A moderator reviews and merges (typically 1-7 days).

### 11.3 Automating with GitHub Actions

Use [winget-releaser](https://github.com/vedantmgoyal9/winget-releaser) to automatically submit
manifest updates when you create a GitHub Release:

```yaml
# .github/workflows/winget-release.yml
name: Publish to winget

on:
  release:
    types: [published]

jobs:
  winget:
    runs-on: ubuntu-latest   # Works on any platform
    steps:
      - uses: vedantmgoyal9/winget-releaser@v2
        with:
          identifier: klivak.ZaiMeter
          installers-regex: 'zaimeter\.exe$'
          token: ${{ secrets.WINGET_TOKEN }}
```

**Setup:**
1. Create a GitHub Personal Access Token (classic) with `public_repo` scope.
2. Store it as a repository secret named `WINGET_TOKEN`.
3. The action uses [Komac](https://github.com/russellbanks/Komac) under the hood to generate
   manifests and submit PRs to winget-pkgs automatically.

### 11.4 Using WingetCreate (Manual Alternative)

Microsoft's official tool for creating and updating winget manifests:

```bash
wingetcreate new https://github.com/klivak/zaimeter/releases/download/v1.10.3/zaimeter.exe
wingetcreate update klivak.ZaiMeter --urls https://github.com/klivak/zaimeter/releases/download/v1.10.3/zaimeter.exe --version 1.10.3
wingetcreate submit <manifest-path> --token <github-pat>
```

### References

- [Submit packages to winget](https://learn.microsoft.com/en-us/windows/package-manager/package/)
- [Create your package manifest](https://learn.microsoft.com/en-us/windows/package-manager/package/manifest)
- [winget-releaser GitHub Action](https://github.com/vedantmgoyal9/winget-releaser)
- [winget-pkgs repository](https://github.com/microsoft/winget-pkgs)

---

## 12. Alternative: Unpackaged EXE Submission

Since late 2021, Microsoft Store accepts unpackaged Win32 applications (raw .exe or .msi)
without MSIX packaging.

### How It Works

1. Host your installer/exe on a publicly accessible URL (e.g., GitHub Releases).
2. In Partner Center, choose "EXE or MSI app" instead of "MSIX or PWA app."
3. Provide the download URL for your installer.
4. Microsoft's certification team downloads and tests the installer.
5. When users click "Install" in the Store, they are directed to download the installer from
   your URL.

### Pros and Cons

| Pros | Cons |
|------|------|
| No MSIX packaging required | Users see a browser download instead of seamless Store install |
| Use your existing .exe as-is | No Store-managed updates |
| Simpler submission process | Less polished user experience |
| Full control over install location | No MSIX sandboxing benefits |

### When to Use This

This is a good option if:
- You want Store discoverability without the MSIX packaging effort.
- You want to start with a simpler submission while working on MSIX packaging.

### References

- [Distribute your Win32 app through Microsoft Store](https://learn.microsoft.com/en-us/windows/apps/distribute-through-store/how-to-distribute-your-win32-app-through-microsoft-store)

---

## 13. Recommended Strategy for ZaiMeter

### Phase 1: winget (Quick Win)

1. Create a winget manifest for the current portable .exe release.
2. Submit a PR to winget-pkgs.
3. Set up the winget-releaser GitHub Action for automatic updates.
4. **Effort:** ~1 hour. **Benefit:** `winget install klivak.ZaiMeter` works for CLI users.

### Phase 2: Microsoft Store (Unpackaged)

1. Register a Partner Center individual account (free).
2. Reserve the "ZaiMeter" app name.
3. Create a Store listing with screenshots, description, and privacy policy.
4. Submit the existing .exe via the unpackaged path.
5. **Effort:** ~1 day (mostly creating assets and writing listings).
6. **Benefit:** Store discoverability without MSIX complexity.

### Phase 3: Microsoft Store (MSIX)

1. Create `AppxManifest.xml` and required visual assets.
2. Build the MSIX packaging into the CI/CD pipeline.
3. Handle MSIX-specific concerns (config file location, auto-start, update mechanism).
4. Submit the MSIX package for a polished Store experience.
5. **Effort:** ~3-5 days (including code changes for MSIX compatibility).
6. **Benefit:** Seamless install/uninstall, Store-managed updates, professional appearance.

### Code Changes Needed for MSIX

- [ ] Add MSIX package detection (`is_msix_packaged()` function).
- [ ] Conditional config/database path (use writable location when packaged).
- [ ] Conditional autostart (skip registry approach when packaged; rely on manifest startup task).
- [ ] Conditional self-updater (disable when packaged; rely on Store updates).
- [ ] Create `AppxManifest.xml` template with proper capabilities.
- [ ] Generate required icon assets at MSIX sizes (44x44, 150x150, etc.).
- [ ] Create `scripts/build-msix.ps1` automation script.
- [ ] Add GitHub Actions workflow for MSIX builds.
- [ ] Host a privacy policy page.

---

## Appendix: Useful Links

- [Partner Center Dashboard](https://partner.microsoft.com/dashboard)
- [Microsoft Store Developer Registration](https://developer.microsoft.com/en-us/microsoft-store/register)
- [MSIX Documentation](https://learn.microsoft.com/en-us/windows/msix/)
- [Windows App Certification Kit (WACK)](https://learn.microsoft.com/en-us/windows/uwp/debug-test-perf/windows-app-certification-kit)
- [Store Policies](https://learn.microsoft.com/en-us/windows/apps/publish/store-policies)
- [winget-pkgs Repository](https://github.com/microsoft/winget-pkgs)
- [cargo-wix](https://github.com/volks73/cargo-wix)
- [MSIX Hero](https://msixhero.net/) -- Free GUI tool for inspecting and editing MSIX packages
