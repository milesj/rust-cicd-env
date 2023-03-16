# cd_env

![Crates.io](https://img.shields.io/crates/v/cd_env) ![Crates.io](https://img.shields.io/crates/d/cd_env)

Detects CD (continuous deploy, deliver, distribute) information from the current environment.

### Usage

To start, detect if in a CD or general deploy environment.

```rust
cd_env::is_cd();
```

Or detect which CD provider is being used.

```rust
cd_env::detect_provider(); // Render
```

And most importantly, extract information about the deploy environment and CD provider.

```rust
use cd_env::get_environment;

if let Some(cd) = get_environment() {
	println!("Provider: {:?}", cd.provider);
	println!("Branch: {}", ci.branch);
	println!("Commit: {}", ci.revision);
}
```

> VCS information isn't always available depending on the provider.
