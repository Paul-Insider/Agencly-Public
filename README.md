# AGENCLY

**Your identity. Your app. Your guest experience.**

AGENCLY is a white-label application platform for businesses that want a digital experience of their own. Its first product focus is Property: a branded residence experience connecting presentation, local discovery and guest services.

This public repository introduces the project, its demonstrations and its development direction, alongside a small, runnable Rust example. The commercial application is developed in a separate private repository.

## Experience the demos

| Demo | Setting | Open the experience |
| --- | --- | --- |
| MayaLuxHome | Caribbean residence | [mayaluxhome.pages.dev](https://mayaluxhome.pages.dev/) |
| DAR OLIVA | Marrakech residence | [daroliva.pages.dev](https://daroliva.pages.dev/) |

Explore the collection on [Try the Apps](https://agencly.com/try-the-apps).

These are demonstration experiences. They are not live property booking services and do not establish the commercial availability of the platform.

## Inside this repository

- [Demo guide](DEMOS.md) — experiences, installation pages and what to explore.
- [Architecture and roadmap](ROADMAP.md) — the product direction and the work ahead.
- [Rust example](src/lib.rs) — typed validation of residence branding, with unit tests and no third-party dependencies.

## Run the Rust example

With an existing Rust toolchain:

```sh
cargo test --offline
```

The example validates a public residence identifier, name and destination. Its contract is documented in the source and exercised by eight tests. It is an independent educational example; it does not expose the commercial service implementation or provide authentication, tenant authorization, booking or payment behavior.

## Built with a clear foundation

Rust and PostgreSQL form the target service foundation, with TypeScript and React for the interface. Web and installable web experiences are the initial distribution focus.

The platform is in active development. Demo availability and production qualification are tracked separately.

## Explore further

[AGENCLY](https://agencly.com/) · [Paul Insider](https://paulinsider.com/) · [GitHub](https://github.com/Paul-Insider)
