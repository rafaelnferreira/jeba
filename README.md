# jeba

**J**ira **e**xtension **ba**se - **jeba** - is a CLI tool to harness jira capabilities.

The motivation come from the fact I process text files containing ticket references, and I got annoyed to use another tools/websites to quickly find what those mean and where they are.

Although **jeba** has been conceived to primarily fits **this gap**, it is completely extensible to fit **others** as well in the future.

## Running locally

You need a [Rust](https://www.rust-lang.org/learn/get-started) environment.

1. Clone the repo and install this package locally:

```bash
cargo install --path .
```

2. Create a file under `$HOME/.jeba/conf.toml` containing your jira access details:

```toml
[jira]
username = "user"
api_token = "token"
site = "https://yourjirasite.atlassian.net"
```

## Usage sample

jeba will capture the input from `stdin`, so use it with any pipe operation and it will attempt to extract a ticket reference and get a summarized information from the Jira api. Example:

```bash
echo -e "hi there is a reference on this line PRJ-0001\nin this line there's no reference" | jeba                      
hi there is a reference on this line line PRJ-0001    [PRJ-0001] Ticket title  / Unassigned / Open
in this line there's no reference
```