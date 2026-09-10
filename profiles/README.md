# Profiles

By-reference profile documents go here: each one a standalone `.ttl` file
rooted at a fixed `#profile` fragment (`odrl:Profile` plus this repo's
`dsc:dutyMode`/`dsc:behaviour`/`dsc:partyIdentityClaim`/
`dsc:agreementAssigneeClaim` extensions), so a test case's
`dsc:profile` can point at `<path/to/file>#profile` instead of repeating
the same profile inline in every case that shares it. None are published
here yet — the one worked test case in `cases/` uses an inline profile.
See `docs/vocabulary-spec.md` for the by-reference convention.
