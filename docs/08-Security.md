# 08 · Security

Security and privacy are built in: no advertisements, no data clutter, and clear
profile boundaries.

## User Profiles

| Profile | Purpose |
|---------|---------|
| Kids | Restricted content and apps |
| Guest | Temporary, isolated session |
| Personal | Standard user |
| Developer | Developer Mode + debugging access |
| Administrator | Full device control |

Each profile independently stores: Apps · History · Preferences · Wallpaper ·
Settings.

## Settings Surface

The Settings → Security section governs device-level protections. Related
surfaces: Accounts, Applications (permissions), and Developer.

## Principles

- **Least privilege.** Developer/debug capabilities (ADB, profilers) are gated
  behind the Developer profile and Developer Mode.
- **Isolation.** Guest and Kids profiles are sandboxed from personal data.
- **Verified boot.** The boot pipeline ([Kernel](05-Kernel.md)) establishes a
  trusted system image; OTA updates are signed (see [OTA](09-OTA.md)).
- **No ads, no tracking clutter.** Privacy is a design constraint, not a setting.

## Updates & Integrity

OTA packages are signed and verified before install, with rollback protection.
See [OTA](09-OTA.md).
