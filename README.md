# Cloud Foundry Rust Buildpack

Super simple proof of concept online buildpack for Rust on Cloud Foundry.

### Getting it onto your Cloud Foundry


```bash
$ pwd
/Users/pivotal/workspace/rust-buildpack

$ cf_exec buildpack-packager --uncached
Uncached buildpack created and saved as /Users/pivotal/workspace/rust-buildpack/rust_buildpack-v0.0.1.zip with a size of 120K

$ cf create-buildpack rust-buildpack rust_buildpack-v0.0.1.zip 1
Creating buildpack rust-buildpack...
OK

Uploading buildpack rust-buildpack...
Done uploading
OK
```

### Running the Test Suite

```bash
BUNDLE_GEMFILE=cf.Gemfile buildpack-build --uncached
```
