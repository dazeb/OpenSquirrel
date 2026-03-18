# Linux Packaging

OpenSquirrel ships Linux packaging assets separately from the macOS bundle flow.

## Layout

- `AppRun`: runtime entrypoint used by AppImage
- `opensquirrel.desktop`: desktop launcher metadata
- `AppDir/`: staging tree populated by the packaging script

## Build

Use the scripts in `scripts/`:

```bash
./scripts/build-linux-release.sh
./scripts/build-appimage.sh
```

The AppImage build emits artifacts into `dist/`.
