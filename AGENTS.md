- If you are failing to do a task stop, don't try to go around the problem. For example if the tests are failing do not remove the tests.

These are the md files that you should read:

- DIOXUS.md If you have to do anything with dioxus
- LIVING_PLAN.md this defines all the phases of the project
- PROJECT.md this explains what the project is supposed to do
- WIREFRAMES.md this explains what the project is supposed to look like

If you are implementing a design feature, follow the guidelines in the WIREFRAMES.md file.

Critical Rules
1. Code Organization
    Many small files over few large files
    High cohesion, low coupling
    200-400 lines typical, 800 max per file
    Organize by feature/domain, not by type

2. Code Style
    No emojis in code, comments, or documentation
    Immutability always - never mutate objects or arrays
    No console.log in production code
    Proper error handling with try/catch
    Input validation with Zod or similar

Assume I'm always running `dx serve`, no need to run `dx build`, `cargo check` is enough.

Do not create other md files unless requested
