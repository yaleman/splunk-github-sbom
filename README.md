# splunk-github-sbom

Send your SBOM data to a Splunk instance via the HTTP Event Collector endpoint.

<!-- arguments table start -->
| Argument   | Description                                  | Required | Default |
| ---------- | -------------------------------------------- | -------- | ------- |
| data       | A JSON event to send                         | true     |         |
| index      | Set the index                                | false    |         |
| owner      | The username/organisation that owns the repo | true     |         |
| port       | Set the HEC port                             | false    | 8088    |
| repository | The repo name                                | true     |         |
| server     | The Splunk server to send to                 | true     |         |
| source     | Set the source                               | false    |         |
| sourcetype | Set the sourcetype                           | false    |         |
| token      | The HEC token to use                         | true     |         |
<!-- arguments table end -->

## Troubleshooting things

- Add "SHOW_RATELIMIT=1" to the environment variables, if you're worried about getting rate-limited.

## TODO

- [ ] Update this README with the details of this action
- [ ] Rename the default Git branch to `v1` (instead of `main` or `master`. This helps with potential future breaking changes. **PROVIDED ACTIONS WILL NOT WORK UNTIL YOU DO THIS**
