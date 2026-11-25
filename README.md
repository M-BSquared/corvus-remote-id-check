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
and built into a WebAssembly module that every system runs, published from this repository. It has no dependency of its own.

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

Commit, then:

```bash
./release.sh
```

It runs the tests and pushes `main`. The workflow builds the plugin once, with the oldest Corvus
GCS that `plugin.json` names (`min_corvus`) or the first published one after it, lets that program
load it, and attaches it to the release `plugin` of this repository. Every later Corvus GCS takes
it as it is. Raise `min_corvus` when the plugin starts to need something newer: older stations
then go on getting the last version that fits them. `plugin.json` also names the permissions the
plugin uses, which the operator sees before installing it. `--watch` waits for the result, `--help`
lists the rest.

## Tests

From the Corvus checkout:

```bash
cargo test -p corvus_plugin_remote_id_check
```

## Licence

Sustainable Use License, see [LICENSE.md](LICENSE.md). Copyright (c) 2026
Maximilian Böck. The tables in `src/standard.rs` are MIT, Copyright (c) 2026
Matthew Busel.
