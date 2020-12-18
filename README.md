# kubectl-akri

Very early (!) prototype of schema validation for Akri.

To avoid: https://github.com/deislabs/akri/issues/177

```bash
cargo run
Error: ValidationError("#.spec.brokerPodSpec.containers[0]: field \'env\' is not specified in the schema\n")
```

Unfortunately, because, it's necessary to define as must-exist not optional extras.

