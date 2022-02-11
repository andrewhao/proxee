# proxee

A zero-config CLI reverse proxy for local SPA and backend development

<img width="962" alt="image" src="https://user-images.githubusercontent.com/222583/153681875-b6ede0b7-ab4e-464d-ab25-f5653c7e2385.png">

Local SPA development is hard, especially if you need to proxy API requests to a cloud- or locally-hosted API backend. `proxee` makes this easier with a CLI-based reverse proxy server so you can locally develop against multiple local- or cloud-based services.

### Usage

    $ proxee start

### Config

Set up a `.proxee.json` file in the root directory where you want to run the proxy

```json
{
  "hosts": {
    "<HOST_IDENTIFIER>": "<URL_TO_SERVICE>"
  },
  "rules": {
    "<PATH_REGEX>": "<HOST_IDENTIFIER>"
  }
}
```

### Example: Two services, different subdomains:

Let's say we have two services, `"main"` and `"blog"`. One serves the site at `www.mydomain.test`, and the other runs on `blog.mydomain.test` subdomain. "main" runs on localhost:3000 and "blog" runs on localhost:3001.

Your config would be:

```json
{
  "hosts": {
    "main": "http://localhost:3000",
    "blog": "http://localhost:3001"
  },
  "rewrites": {
    "^http://blog.mydomain.test/*$": "blog" // Route ui.mydomain.com to locally-running `ui` microservice
    "^http://www.mydomain.test.*$": "main", // Fallback to `app` microservice
  }
}
```

### Example: Two services, one mounted as subdirectory

Let's say we have two services, `"main"` and `"blog"`. One serves the site, and the other is "mounted" at the `/blog` path. "main" runs on localhost:3000 and "blog" runs on localhost:3001.

Your config would be:

```json
{
  "hosts": {
    "main": "http://localhost:3000",
    "blog": "http://localhost:3001"
  },
  "rewrites": {
    "^http://www.mydomain.test/blog.*$": "blog" // Rewrite all requests to the /blog directory to "blog" service
    "^http://.*$": "main", // Fallback to `main` microservice
  }
}
```

### Static resources (i.e. Next.js application)

If you use special frameworks like Next.js, you will need to also redirect special Next.js paths. These are easily accomplished with the right regex expressions:

```json
{

  "rewrites": {
    "...": "...",
    "^http://www.mydomain.test/_next.*$": "blog", // Special frameworks like Next.js also have reserved paths for fetching static data. You may want to require this if you need to route asset requests to your SPA.
    "...": "...",
  }
}
