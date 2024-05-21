# Hitler Crawler

This app takes link to a Wikipedia page, fetches it, extracts links from the article (not all links on the page), filters them and if it finds destination page (defaults to Adolf Hitler's page on Wikipedia) if stops, otherwise it repeats the same steps for each link it finds before it reaches the destination page or reaches maximal height (defaults to 6).

Pages are loaded and parsed asynchronously using mainly `reqwest` and `tokio`. There are several options to configure. You can use `cargo run -- --help` to display them all.

```
  -o, --origin <ORIGIN>            Origin to start crawling
  -d, --destination <DESTINATION>  Destination to crawl to. Defaults to "https://wikipedia.org/wiki/Adolf_Hitler"
  -t, --timeout <TIMEOUT>          Reqwest timeout in seconds. Defaults to 20
  -m, --max-depth <MAX_DEPTH>      Maximal depth to crawl. Defaults to 6
  -l, --limit <LIMIT>              Limit of recuests per second. Defaults to 20
  -f, --fast <FAST>                If true - program will create a database and check if any reached link is contained in database. If database contains link, it will check if path is still reachable and return it. This may lead to non-optimal (not shortest length) paths. Defaults to true [possible values: true, false]
```

Example of use: `cargo r -r -- --origin https://en.wikipedia.org/wiki/Algeria --limit 30 --timeout 10`

I had some problems with building `openssl` on my machine, so I've used `nix` to fix that. You can build it in the same way by running `nix-shell` before building the project.
