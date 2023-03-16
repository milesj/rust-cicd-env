# cd_env

![Crates.io](https://img.shields.io/crates/v/cd_env) ![Crates.io](https://img.shields.io/crates/d/cd_env)

Detects CD (deploy) information from the current environment.

### Usage

To start, detect if in a CD or general deploy environment.

```rust
cd_env::is_cd();
```

Or detect which CD provider is being used.

```rust
cd_env::detect_cd_provider(); // Render
```

And most importantly, extract information about the deploy environment and CD provider.

```rust
use cd_env::get_deploy_environment;

if let Some(ci) = get_cd_environment() {
	println!("Provider: {:?}", ci.provider);
	println!("Branch: {}", ci.branch);
	println!("Commit: {}", ci.revision);

	if let Some(pr_number) = ci.request_id {
		println!("PR #: {}", pr_number);
	}
}
```
