# AGENCLY

**Your brand. Your apps. Your agency.**

AGENCLY is a multi-sector white-label application platform for businesses and partner agencies. Its scope spans eCommerce, restaurants, courses, travel, Property and other business applications, with each experience shaped around the identity of the business it serves.

AGENCLY combines two complementary offerings: branded applications for businesses, and a white-label agency model for partners who want to build their own application offering with us.

This public repository presents that direction, the available demonstrations and an independent Rust example. The commercial platform is developed in a separate private repository.

## One platform, many business sectors

| Sector | Example application scope |
| --- | --- |
| Property and hospitality | Residences, accommodation presentation and guest experiences |
| eCommerce | Branded stores, product catalogues and shopping journeys |
| Restaurants and coffee shops | Menus, venue presentation, reservations and ordering experiences |
| Courses and education | Course catalogues, programmes and enrolment journeys |
| Travel agencies and tours | Destinations, itineraries, excursions and reservation experiences |
| Beauty and wellness | Service presentation and appointment journeys |
| Sport and fitness | Classes, coaching, sessions and memberships |
| Automotive | Vehicle catalogues, enquiries and showroom appointments |
| Other business applications | Additional use cases defined through an agreed project scope |

These categories describe AGENCLY's platform scope. The [roadmap](ROADMAP.md) and [demo guide](DEMOS.md) identify the experiences currently available to explore. Property is the first commercial qualification milestone within this broader platform.

## For businesses

Create an application around your own name, logo, visual identity, content and business needs. The white-label approach applies across sectors; functionality and delivery are defined for the selected application and agreed scope.

App subscriptions and app acquisition are distinct commercial paths. Their rights, services, operating costs and delivery arrangements are set out in the applicable agreement.

## For partner agencies

Work with AGENCLY to establish an application offering under your own agency identity. The partnership model brings together an agency's branded customer-facing storefront and sales materials with white-label applications delivered for its clients.

There are two levels of identity: the partner agency presents its own offering, and each client's application carries that client's brand. The agreed customer-facing surfaces can be presented without visible AGENCLY branding; product rights, responsibilities and operating arrangements are defined in the partnership agreement.

[Explore the partner-agency model](PARTNERS.md).

## Experience the current demos

| Demo | Category | Setting | Open the experience |
| --- | --- | --- | --- |
| MayaLuxHome | Property | Caribbean residence | [mayaluxhome.pages.dev](https://mayaluxhome.pages.dev/) |
| DAR OLIVA | Property | Marrakech residence | [daroliva.pages.dev](https://daroliva.pages.dev/) |

Explore the collection on [Try the Apps](https://agencly.com/try-the-apps). These two demonstrations illustrate the first Property experiences within AGENCLY's multi-sector direction. Additional demonstrations will be documented as they become available.

The demos are for evaluation. They are not live property booking services and do not establish the commercial availability of every platform journey.

## Inside this repository

- [Demo guide](DEMOS.md) — experiences, installation pages and what to explore.
- [Partner agencies](PARTNERS.md) — the white-label agency model and collaboration framework.
- [Architecture and roadmap](ROADMAP.md) — the shared foundation and development milestones.
- [Rust example](src/lib.rs) — typed residence-branding validation, with unit tests and no third-party dependencies.

## Run the Rust example

With an existing Rust toolchain:

```sh
cargo test --offline
```

The public example validates a residence identifier, name and destination through eight tests. It illustrates one small branding boundary within the broader platform. It is independent of the commercial implementation and does not provide authentication, tenant authorization, booking or payment behavior.

## Built with a shared foundation

Rust and PostgreSQL form the target service foundation, with TypeScript and React for the interface. Shared services are intended to support distinct application models and brand identities. Web and installable web experiences are the initial distribution focus.

The platform is in active development. Demonstrations, partner onboarding and production qualification have their own documented scopes and milestones.

## Rights and legal information

Copyright (c) 2026 Paul Insider. All rights reserved. The [LICENSE](LICENSE) permits personal or internal evaluation and testing of the public example; other uses require separate permission. This repository is publicly visible and is not released under an open-source licence.

See the [legal information](legal/README.md), [brand notice](legal/TRADEMARKS.md) and [third-party notices](legal/THIRD_PARTY_NOTICES.md). Rights provided by applicable law and GitHub's applicable terms remain unaffected.

## Explore further

[AGENCLY](https://agencly.com/) · [Paul Insider](https://paulinsider.com/) · [GitHub](https://github.com/Paul-Insider)
