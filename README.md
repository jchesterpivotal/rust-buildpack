# Cloud Foundry Rust Buildpack

Super simple proof of concept online buildpack for Rust on Cloud Foundry.

### Getting it onto your Cloud Foundry

```bash
$ pwd
/Users/pivotal/workspace/rust-buildpack

$ buildpack-packager --cached 
Downloading rust version 1.10.0 from: file:///Users/pivotal/workspace/rust-output/rust-1.10.0.tgz
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100  232M  100  232M    0     0   292M      0 --:--:-- --:--:-- --:--:--  279M
  Using rust version 1.10.0 with size 233M
  rust version 1.10.0 matches the manifest provided md5 checksum of 1460714f450f3f1d2ff2032acdcbb436

Cached buildpack created and saved as /Users/pivotal/workspace/rust-buildpack/rust_buildpack-cached-v0.0.2.zip with a size of 106M

$ cf create-buildpack rust-buildpack rust_buildpack-v0.0.2.zip 1
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
