# Cloud Foundry Rust Buildpack

Super simple proof of concept online buildpack for Rust on Cloud Foundry.

### Getting it onto your Cloud Foundry

```bash
$ pwd
/Users/pivotal/workspace/rust-buildpack

$ zip -r rust-buildpack-v0.0.1.zip ./bin

$ cf create-buildpack rust-buildpack rust_buildpack-v0.0.1.zip 1
Creating buildpack rust-buildpack...
OK

Uploading buildpack rust-buildpack...
Done uploading
OK
```

### Running a test app
```bash
$ cd ./cf_spec/fixtures/rust_app_hello_world_server

$ cf push
```
