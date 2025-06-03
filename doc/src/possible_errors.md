# Possible errors

This chapter describes potential errors that can occur due to misconfiguration
either on the user side or on the provider side.

## Service catalog and version discovery

It could happen, that the URL contained in the service endpoint version
document point to the URL that is either malformed or is simply wrong (i.e.
returning 404). While the SDK may be able to cope with certain errors not
always the result is going to be correct. This type of errors must be fixed by
the cloud provider. User should be able to set the
`<SERVICE>_endpoint_override` in the `clouds.yaml` to temporarily workaround
issues.

### Invalid port

The host port must be a valid port.

### Absolute path

The URL must be an absolute path. Official procedure of the version discovery
supports use of the relative URLs, but this is not used by the official
services and should never happen.

### Format

The URL must match the following regular expression:
`"^(?<scheme>.+)://(?<host>[^:]+):(?<port>[^/]+)/(?<path>.*)$`

### Unreachable url

The URL in the version document must be a working url. 

- `https://example.com//v2`

- `http://localhost:8080/invalid_prefix/v2`

SDK ignores the information as if at the attempted url nothing usable was found
at all. Reason for that is that it indicates major misconfiguration on the
provider side and most likely all other links in the API would be broken as
well (pagination, self links, etc).

### Version url cannot be a base

The URL must be a valid URI.

Wrong:

- `ftp://rms@example.com`
- `unix:/run/foo.socket`
- `data:text/plain,Stuff`

### Discovered URL has different prefix

Under some misconfiguration circumstances the discovery document may point to a
pretty different location compared to the place where the discovery document
itself has been found (i.e. discover points to `http://localhost/v2.1` while
the document itself has been found at `http://localhost/prefix/v2.1`). This is
a service misconfiguration that need to be addressed by the cloud provider. In
case of errors on the client side configuring `<SERVICE>_endpoint_override` may
or may not help.

It is not possible to simply validate discovered url because some services are
not implementing discovery properly. Due to that fact SDK cannot do anything
else than just note a warning.
