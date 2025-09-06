# Authentication and authorization

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

## Configuration

Rust based tools support typical
[`clouds.yaml`/`secure.yaml`](https://docs.openstack.org/openstacksdk/latest/user/config/configuration.html)
files for configuration.

Most authentication methods support interactive data provisioning. When certain
required auth attributes are not provided in the configuration file or through
the supported cli arguments (or environment variables) clients that implement
`AuthHelper` interface can receive such data from the user. For example when
using the cli a prompt will appear. In the tui a popup requests the user input.

### Authentication methods

Currently only a subset of all possible authentication methods is covered with
the work on adding further method ongoing

#### v3Token

A most basic auth method is an API token (`X-Auth-Token`). In the `clouds.yaml`
this requires setting `auth_type` to one of the [`v3token`, `token`]. The token
itself should be specified in the `token` attribute.

#### v3Password

A most common auth method is a username/password. In the `clouds.yaml` this
requires setting `auth_type` to one of the [`v3password`, `password`] or
leaving it empty.

Following attributes specify the authentication data:

- `username` - The user name
- `user_id` - The user ID
- `password` - The user password
- `user_domain_name` - The name of the domain user belongs to
- `user_domain_id` - The ID of the domain user belongs to

It is required to specify `username` or `user_id` as well as `user_domain_name`
or `user_domain_id`.


#### v3Totp

Once user login is protected with the MFA a OTP token must be specified. It is
represented as `passcode`, but it not intended to be used directly in the `clouds.yaml`

#### v3Multifactor

A better way to handle MFA is by using a `v3multifactor` auth type. In this
case configuration looks a little bit different:

- `auth_type` = `v3multifactor`
- `auth_methods` is a list of individual `auth_type`s combined in the
authentication flow (i.e `['v3password', 'v3totp']`)

When a cloud connection is being established in an interactive mode and server
responds that it require additional authentication methods those would be
processed based on the available data.

#### v3WebSso

An authentication method that is getting more a more popular is a Single Sign
On using remote Identity Data Provider. This flow requires user to authenticate
itself in the browser by the IDP directly. It is required to provide following
data in the configuration in order for this mode to be used:

- `auth_type` = `v3websso`
- `identity_provider` - identity provider as configured in the Keystone
- `protocol` - IDP protocol as configured in the Keystone

**Note:** This authentication type only works in the interactive mode. That
means in the case of the CLI that there must be a valid terminal (`echo foo |
osc identity user create` will not work)

#### v3ApplicationCredential

Application credentials provide a way to delegate a user’s authorization to an
application without sharing the user’s password authentication. This is a
useful security measure, especially for situations where the user’s
identification is provided by an external source, such as LDAP or a
single-sign-on service. Instead of storing user passwords in config files, a
user creates an application credential for a specific project, with all or a
subset of the role assignments they have on that project, and then stores the
application credential identifier and secret in the config file.

Multiple application credentials may be active at once, so you can easily
rotate application credentials by creating a second one, converting your
applications to use it one by one, and finally deleting the first one.

Application credentials are limited by the lifespan of the user that created
them. If the user is deleted, disabled, or loses a role assignment on a
project, the application credential is deleted.

Required configuration:

- `auth_type` = `v3applicationcredential`
- `application_credential_secret` - a secret part of the application credential
- `application_credential_id` - application credential identity
- `application_credential_name` - application credential name. **Note:** It is
required to specify user data when using application credential name
- `user_id` - user ID when `application_credential_name` is used
- `user_name` - user name when `application_credential_name` is used
- `user_domain_id` - User domain ID when `application_credential_name` is used
- `user_domain_name` - User domain ID when `application_credential_name` is used

Either `application_credential_id` is required or `application_credential_name`
in which case additionally the user information is required.

#### v3OidcAccessToken

Authentication with the OIDC access token is supported in the same way like it
is done by the python OpenStack tools.

Required configuration:

- `auth_type` = `v3oidcaccesstoken` (or `oidaccesstoken`)
- `identity_provider_id` - an identity provider id
- `protocol` - identity provider protocol (usually `oidc`)
- `access_token` - Access token of the received from the remote identity
provider.


## Caching

As described above in difference to the Python OpenStack tooling authentication
caching is enabled by default. It can be disabled using `cache.auth: false` in
the `clouds.yaml`.

Data is cached locally in the `~/.osc` folder. It is represented by set of
files where file name is constructed as a hash of authentication information
(discarding sensitive data). Content of the file is a serialized map of
authorization data (scope) with the token information (catalog, expiration,
etc).

Every time a new connection need to be established first a search in the cache
is performed to find an exact match using supplied authentication and
authorization information. When there is no usable information (no information
at all or cached token is already expired) a search is performed for any valid
token ignoring the scope (authz). When a valid token is found in the cache it
is used to obtain a new authorization with required scope. Otherwise a new
authentication is being performed.
