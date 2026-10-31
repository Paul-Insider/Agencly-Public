# Architecture and roadmap

AGENCLY is being developed as a white-label platform, starting with a single-property application. This page describes direction; it is not a release commitment or a list of commercially available features.

## Architecture direction

| Layer | Direction |
| --- | --- |
| Interface | TypeScript and React, with a shared application structure and distinct visual identities |
| Services | A Rust core with explicit application and tenant boundaries |
| Data | PostgreSQL for durable application data |
| Integrations | Provider interfaces for services such as payments and transactional email |
| Distribution | Web and installable web experiences first; native distribution evaluated separately |

The residence's identity and content should be configurable while shared services remain reusable across Property applications.

## Available to explore

- Two independent Property web demonstrations: MayaLuxHome and DAR OLIVA.
- A public demo collection on the AGENCLY website.
- Public documentation in this repository.
- An independent Rust branding-validation example with unit tests and no third-party dependencies.

## In progress

- Qualification of the Rust service foundation and end-to-end Property journeys.
- Owner configuration and the connection of configured content to the published experience.
- Localization coverage, contextual review and interface verification.
- Consistent accessibility and behavior across desktop and mobile views.

## Next milestones

1. **Assisted Property delivery:** configure and qualify a first complete application for an owner.
2. **Client self-service:** guide a customer through configuration, review and web publication.
3. **Broader reuse:** apply the qualified shared foundation to additional Property identities.
4. **Distribution expansion:** assess native packages and their signing and store requirements separately from web installation.

Progress will be documented as individual milestones are verified. A demo, a unit test result and a production-qualified customer journey represent different stages of completion.

[Back to AGENCLY](README.md) · [Explore the demos](DEMOS.md)
