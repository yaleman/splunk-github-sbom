# splunk-github-sbom

Send your SBOM data to a Splunk instance via the HTTP Event Collector endpoint.

There's an example of usage [in our own github actions test file](https://github.com/yaleman/splunk-github-sbom/blob/dev/.github/workflows/test_thyself.yml), but here's another option:

You'll need secrets for at least the target server and HTTP Event Collector token.

```yaml
- uses: yaleman/splunk-github-sbom@v1
  with:
    github_token: "${{ github.token }}"
    server: "${{ secrets.SERVER }}"
    splunk_token: "${{ secrets.TOKEN }}"
    repository: "${{ github.repository }}"
```

<!-- arguments table start -->
| Argument     | Description                                  | Required | Default        |
| ------------ | -------------------------------------------- | -------- | -------------- |
| github_token | The GitHub token to use                      | true     |                |
| index        | Set the index                                | false    |                |
| port         | Set the HEC port                             | false    | 8088           |
| repository   | The full repo name, ie 'yaleman/hello-world' | false    |                |
| server       | The Splunk server to send to                 | true     |                |
| source       | Set the source                               | false    | github-actions |
| sourcetype   | Set the sourcetype                           | false    | github:sbom    |
| splunk_token | The HEC token to use                         | true     |                |

<!-- arguments table end -->

## References

- [How to write a github action in Rust](https://dylananthony.com/blog/how-to-write-a-github-action-in-rust/)
