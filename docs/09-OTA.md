# 09 · OTA Updates

Over-the-air updates keep LiteTV OS current while protecting device integrity and
preserving user data.

## Goals

- Signed, verified update packages
- Safe rollback on failure
- Minimal download size (system image under 2.5 GB baseline)
- OTA history retained and backed up via [LiteCloud](10-Ecosystem.md)

## Flow

1. **Check** — device polls for an available update.
2. **Download** — package fetched in the background, throttled by
   [power mode](07-Power-Optimization.md).
3. **Verify** — signature and integrity checked (see [Security](08-Security.md)).
4. **Stage & Apply** — applied on next idle/reboot.
5. **Rollback** — automatic revert if the new image fails verified boot.

## History & Backup

OTA history is part of the [LiteCloud](10-Ecosystem.md) backup set alongside
settings, installed apps, favorites, wallpaper, and user profiles. Update status
is surfaced in **Settings → Updates**.
