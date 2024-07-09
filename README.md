# ComponentizeJS multiworld example

## Intro

This is an example of merging 2 seperate worlds from different packages and building an app that targets the union of the worlds. The 2 npm packages here are

- `@fermyon/spin-sdk` using a git reference to https://github.com/karthik2804/spin-js-sdk/tree/configure_componentizejs
- `@fermyon/wasi-exit` sing a git reference to https://github.com/karthik2804/js-wasi-ext/tree/configure_componentizejs

The `spin-sdk` package has the following main `wit` file and the SDK package is built depending on the imports of the `spin-imports` world. 

```
package fermyon:spin@2.0.0;

world spin-imports {
  import wasi:http/outgoing-handler@0.2.0;
  import llm;
  import redis;
  import postgres;
  import mysql;
  import mqtt;
  import sqlite;
  import key-value;
  import variables;
}

world spin-http {
  include spin-imports;
  export wasi:http/incoming-handler@0.2.0;
}
```

The `wasi-ext` package has the following main `wit` file and the sdk is built depending on the imports of the `js-wasi-ext` world

```
package fermyon:js-wasi-ext@0.2.0;

world js-wasi-ext {
    include wasi:cli/imports@0.2.0;
}
```

For the application to work, the target world will need to be a union of the above worlds. 

## How it works

- Both the npm packages have a `postinstall` script that creates/adds details about where to find the `wit` files to `componentizejs.json` config file after installing them. 
- The `wit_path_extractor` is used to resolve the paths of the wit files for each of the packages. 
- the `knitwit` binary combines the required worlds and outputs a new `combined_wit` folder with the union of the two worlds. 

## Prereqs

- Spin
- Rust
- Node.js

## Building and running the example.

1. Build the `knitwit` binary
   
   ```bash
   (cd knitwit && cargo build --release)
   ```

2. Install dependencies for the JS app and check if the `componentizejs.json` file is created.
    ```bash
    cd js-app
    npm install
    cat componentizejs.json
    # [{"name": "@fermyon/wasi-ext", "witPath": "../wit"}, {"name": "@fermyon/spin-sdk", "witPath": "../bin/wit"}]
    ```

3. Use `knitwit` to create a new world "combined" that is a union of the provided worlds
    ```bash
    ../knitwit/target/release/knitwit --output-world combined ${node wit_path_extractor} --world spin-http --world js-wasi-ext
    ```

4. Now build the spin app using `spin build` which will invoke `npm run build` which in turn calls a script called `j2w` which is just a wrapper around `componentize` function exported by `@bytecodealliance/ComponentizeJS`. The arguments passed include the directory of the combined `wit` and targets the `spin-http` world which now also includes the `js-wasi-ext` world. 

    ```bash
    spin build
    ```

5. Run the application and set some environment variables to test
    ```bash
    spin up -e FOO=BAR
    ```

6. Test the application by using `curl`
    ```bash
    curl localhost:3000
    ```