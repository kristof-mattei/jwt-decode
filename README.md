# jwt-decode

Use:

```
echo "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c" | jwt-decode
{
  "alg": "HS256",
  "typ": "JWT"
}
{
  "iat": 1516239022,
  "name": "John Doe",
  "sub": "1234567890"
}
```

If using via Docker:

```
echo "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c" | docker run -i jwt-decode
{
  "alg": "HS256",
  "typ": "JWT"
}
{
  "iat": 1516239022,
  "name": "John Doe",
  "sub": "1234567890"
}
```

## License

MIT, see [LICENSE](./LICENSE)

`SPDX-License-Identifier: MIT`
