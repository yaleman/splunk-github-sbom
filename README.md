# splunk-github-sbom

Send your SBOM data to a Splunk instance via the HTTP Event Collector endpoint.

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
