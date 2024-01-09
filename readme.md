# Saphire-typegen

This crate is part of the saphire project, for more info see Saphire-rs.

This crate is split into 2 halves, one for types to help serialize and deserialize [Diamondfire's](mcdiamondfire.net) actiondump, and the other half provides type generation from said actiondump for Saphire-types.
This crate is mostly intended to be used internally for Saphire-types, but a 100% parsing capability is the main priority.

## Getting started

Deserializing the actiondump is as simple as using serde-json to deserialize an action. It is important to note that dynamic actions (ie. "call function" and "start process") are not supported yet and will error if you try to deserialize them.

If you wish to make use of generated code please instead use Saphire-types.

// TODO: Finish README.md