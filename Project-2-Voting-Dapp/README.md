sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"

export PATH="/Users/misterzurg/.local/share/solana/install/active_release/bin:$PATH"

solana --version
solana-cli 2.1.15 (src:53545685; feat:3271415109, client:Agave)

cargo install --git https://github.com/coral-xyz/anchor avm --force
avm --version

(base) misterzurg@MacBook-Pro-Denis-2 Solana-Developer-Bootcamp % npx create-solana-dapp
Need to install the following packages:
  create-solana-dapp@4.1.2
Ok to proceed? (y) y
┌  create-solana-dapp 4.1.2
│
◇  Enter project name
│  voting-dapp
│
◇  Select a framework
│  Next.js
│
◇  Select a template
│  next-tailwind-counter


If you're on mac:
U can face the issue

> https://github.com/coral-xyz/anchor/discussions/2831#discussioncomment-8673021
```sh
brew uninstall rust
```

# voting-dapp

## Getting Started

### Prerequisites

Testing app
[votingdapp.spec.ts](/Users/misterzurg/Documents/GitHub/Solana-Developer-Bootcamp/Project-2-Voting-Dapp/anchor/tests/votingdapp.spec.ts)

// For learning purposes we have to manually copy program binary.
1. Create fixtures tests/folder
2. Copy /target/deploy/votingdapp.so
3. Put `const IDL = require("../../target/idl/votingdapp.json")`

```
% anchor test --skip-local-validator --skip-deploy
```
