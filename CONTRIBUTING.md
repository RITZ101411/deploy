# Contributing

## Pull Requests

- Branch from `main`
- Branch name: `feat/description`, `fix/description`, `docs/description`, etc.
- Keep PRs focused — one feature or fix per PR
- Ensure `cargo build` and `cargo test` pass before submitting
- Direct pushes to `main` are blocked; all changes go through PRs

## Commit Messages

```
prefix(crate): content
```

### Prefix

- `feat` — New feature
- `fix` — Bug fix
- `refactor` — Code restructuring without behavior change
- `docs` — Documentation
- `chore` — Build, config, and other maintenance

### Crate

Target crate name: `api`, `cli`, `shared`

Can be omitted when changes span multiple crates.

### Examples

```
feat(api): add project deploy endpoint
fix(cli): correct tar.gz compression excluding node_modules
refactor(api): split K8s operations into service layer
docs: add CONTRIBUTING.md
chore(api): setup crate and add health endpoint
```
