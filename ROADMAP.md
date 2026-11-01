# Architecture and roadmap

AGENCLY's direction is a multi-sector white-label application platform, serving both businesses directly and agencies building their own white-label offering. Property is the first commercial qualification milestone; the broader scope includes eCommerce, restaurants, courses, travel and additional business applications.

This page describes product direction. Commercial scope, release commitments and partnership responsibilities are defined separately for each offering.

## Architecture direction

| Layer | Direction |
| --- | --- |
| Interface | TypeScript and React, with a shared structure and distinct business and agency identities |
| Services | A Rust core with explicit application and tenant boundaries |
| Data | PostgreSQL for durable application data |
| Integrations | Provider interfaces for services such as payments and transactional email |
| Application models | Sector-specific content and journeys supported by reusable shared services |
| Agency model | An agency's branded customer-facing offering and its clients' own branded applications |
| Distribution | Web and installable web experiences first; native distribution evaluated separately |

Branding, content and business configuration should follow the selected application model. Reusable services and clear scope boundaries form the shared foundation across sectors and partner arrangements.

## Available to explore

- Two independent Property web demonstrations: MayaLuxHome and DAR OLIVA.
- A public demo collection on the AGENCLY website.
- Public documentation covering the platform, sectors and partner-agency model.
- An independent Rust branding-validation example with eight unit tests and no third-party dependencies.

## In progress

- Qualification of the Rust service foundation and the first complete Property journeys.
- Owner configuration and the connection of configured content to the published experience.
- Localization coverage, contextual review and interface verification.
- Consistent accessibility and behavior across desktop and mobile views.

## Development milestones

1. **First qualified Property delivery:** configure and qualify a complete application for an owner, with assisted delivery as the initial milestone.
2. **Client self-service:** guide a customer through configuration, review and web publication of a qualified application model.
3. **Multi-sector expansion:** extend and qualify shared services and sector-specific journeys for eCommerce, restaurants, courses, travel and further business applications.
4. **Partner-agency rollout:** define and deliver the agreed agency identity, customer-facing offering, application scope, fulfilment and support responsibilities.
5. **Distribution expansion:** assess native packages and their signing and store requirements separately from web installation.

Milestones will be updated as their respective journeys are verified. Public demonstrations and unit tests provide evidence within their own scope; production qualification is assessed for each application and operating model.

[Back to AGENCLY](README.md) · [Explore the demos](DEMOS.md) · [Partner agencies](PARTNERS.md)
