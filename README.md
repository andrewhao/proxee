# proxee

A zero-config CLI reverse proxy.

Primarily for solving the use case where you're locally developing two apps running on two separate ports and you want to merge them on the same host.

### Usage

    $ proxee start

### Config

Set up a `.proxee.json` file in the root directory where you want to run the proxy

```json
{
  "hosts": {
    "<HOST_IDENTIFIER>": "<URL_TO_SERVICE>"
  },
  "rewrites": {
    "<PATH_REGEX>": "<HOST_IDENTIFIER"
  }
}
```

Example:

Let's say we have two services, `"main"` and `"blog"`. One serves the site, and the other is "mounted" at the `/blog` path. "main" runs on localhost:3000 and "blog" runs on localhost:3001.

Your config would be:

```json
{
  "hosts": {
    "app": "http://localhost:3000",
    "ui": "http://localhost:3001"
  },
  "rewrites": {
    "^http://ui.*$": "ui"
    "^http://.*$": "app",
  }
}
```
