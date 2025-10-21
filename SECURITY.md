# Security Policy

This document describes how to report and handle security issues for **IPL**, the interpreted programming language written in Rust.

---

## Supported Versions

Only the latest release and the `main` branch are supported for security fixes.

| Version | Supported |
|----------|------------|
| `main` (development) | ✅ |
| Latest release tag   | ✅ |
| Older versions       | ❌ |

---

## Reporting a Vulnerability

If you find a security vulnerability, **do not open a public issue or pull request**.

To report it safely:

1. Go to the project’s **GitHub repository**.  
2. Navigate to the **“Security”** tab.  
3. Click **“Report a vulnerability.”**  
4. Fill out all requested information, including:
   - Description of the issue  
   - Steps to reproduce  
   - Expected impact  
   - (Optional) proof of concept

I will review your report and respond privately.

---

## Response Process

1. The report is verified and rated for severity.  
2. A private patch branch is created and tested.  
3. When resolved:
   - A new release is published.  
   - The issue is noted in the changelog.  
   - The reporter is credited if they choose.

---

## Disclosure
Please keep security reports confidential until a fix is released.
Coordinated disclosure protects users and the integrity of the project.

Thank you for helping keep **IPL** secure
