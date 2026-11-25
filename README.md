# Remote ID Check for Corvus GCS

A plugin for [Corvus GCS](https://github.com/M-BSquared/CorvusGCS), the
offline ground control station for PX4 and ArduPilot.

It lists the coded fields of the Remote ID this station sends, names each code
as ASTM F3411 and ASD-STAN do, and says when a code is not one of the standard.
It reads only: the identity, the broadcast and the aircraft are not touched. It is no proof that
an identity is valid.

Install it in Corvus GCS under Settings, Plugins. The full description is in
the [Corvus guide](https://m-bsquared.github.io/CorvusGCS/guide/plugins.html#remote-id-check).

## What it is

A Rust crate (`corvus_plugin_remote_id_check`) built on `corvus_api`, the plugin API of Corvus GCS,
and published as a shared library from this repository. It has no dependency of its own.

- `src/lib.rs`: the plugin, the check and what is drawn.
- `src/standard.rs`: the tables of the standard, copied by hand from
  [remoteid](https://github.com/Mattbusel/remoteid) 0.4.0 by Matthew Busel
  (MIT, [LICENSE-remoteid.txt](LICENSE-remoteid.txt)).
- `lang/`: German, French and the dialect file.

## Change it

Corvus GCS carries this repository as a submodule at `plugins/remote-id-check`, and the
crate is a member of its workspace:

```bash
git clone --recurse-submodules https://github.com/M-BSquared/CorvusGCS.git
cd CorvusGCS
./run.sh
```

Settings, Plugins then offers the copy built there. A dependency must be one
that `Cargo.lock` of Corvus GCS already holds.

## Publish it

Commit and push to `main`. The workflow tests the plugin, builds it for the
newest published Corvus GCS on Linux, macOS and Windows, lets that program load
it, and attaches it to the release `corvus-<version>` of this repository, where
Corvus GCS of that version finds it. A new version of Corvus GCS gets its build
within hours.

## Tests

From the Corvus checkout:

```bash
cargo test -p corvus_plugin_remote_id_check
```

## Licence

Sustainable Use License, see [LICENSE.md](LICENSE.md). Copyright (c) 2026
Maximilian Böck. The tables in `src/standard.rs` are MIT, Copyright (c) 2026
Matthew Busel.
