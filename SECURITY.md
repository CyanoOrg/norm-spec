# Security Policy

## Supported versions

Before the first stable release, security fixes land on `main`. After an RC is
published, the latest published pre-release is also supported; older
pre-releases are not. A locally prepared or hosted-CI candidate is not a
supported public release.

## Reporting a vulnerability

Do not open a public issue for a suspected vulnerability. Prefer GitHub private
vulnerability reporting at
`https://github.com/CyanoOrg/norm-spec/security/advisories/new` once it is
enabled. If that channel is unavailable, email `bravetwo@163.com` with the
subject `norm-spec security report`.

Include the affected revision, impact, a minimal reproduction, and any proposed
mitigation. Remove real credentials and private project data from examples. We
aim to acknowledge reports within five business days and will coordinate
disclosure after a fix or mitigation is available.

Security-sensitive areas include parser resource exhaustion, path traversal,
unexpected file access, validation bypass, unsafe template output, and machine
protocol ambiguity.
