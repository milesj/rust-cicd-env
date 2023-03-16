# ci_env

![Crates.io](https://img.shields.io/crates/v/ci_env) ![Crates.io](https://img.shields.io/crates/d/ci_env)

Detects CI information from the current environment.

### Usage

To start, detect if in a CI environment.

```rust
ci_env::is_ci();
```

Or detect which CI provider is being used.

```rust
ci_env::detect_ci_provider(); // GithubActions
```

And most importantly, extract information about the CI environment and CI provider. This includes branch, commit, and pull request information.

```rust
use ci_env::get_ci_environment;

if let Some(ci) = get_ci_environment() {
	println!("Provider: {:?}", ci.provider);
	println!("Branch: {}", ci.branch);
	println!("Commit: {}", ci.revision);

	if let Some(pr_number) = ci.request_id {
		println!("PR #: {}", pr_number);
	}
}
```

> This library will properly take into account source and target branches when a pull request is involved. Do note though that not all providers provide this information.
