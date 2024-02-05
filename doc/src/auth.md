## Authentication and authorization

Understanding authentication in OpenStack is far away from being a trivial
task. In general authentication and authorization are different things, but
they are mixed into 1 API request. When trying to authenticate user is passing
identification data (username and password or similar) together with the
requested authorization (project scope, domain scope or similar). As a response
to this API request a session Token is being returned to the user that needs to
be always sent with any following request. When authorization scope need to be
changed (i.e. perform API in a scope of a different project) a re-authorization
need to be performed. That may be done with the same user identification data
or using existing session token.

Existing python based OpenStack tools are keeping only one active session going
through re-authorization whenever required. There is a support for the token
caching which is bound to the used authorization request. When a new session is
requested a search (when enabled) is being performed in the cache and a
matching token is being returned which is then retried. When MFA or SSO are
being used this process is introducing a very ugly user experience forcing user
to re-entrer verification data every time a new scope is being requested with a
new session.

In this project authentication and authorization in user facing applications is
handled differently. Auth caching to the file system is enabled by default.
User identification data (auth_url + user name + user domain name) is combined
into the hash which is then used as a 1st level caching key and points to the
hashmap of authorization data and corresponding successful authorization
response data. A key in the 2nd level is a hash calculated from the scope
information and a value is a returned token with catalog and expiration
information. This way of handling information allows to immediately retrieve
valid auth information for the requested scope if it already exists (which may
be even shared between processes) or reuse valid authentication data to get new
valid requested authorization saving user from need to re-process MFA or SSO
requirements.
