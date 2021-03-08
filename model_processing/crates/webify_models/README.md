# webify_models

An app to convert Gazebo models to a "webified" version (really just takes PNGs and converts them over, and then moves around the image references)

## Testing

For unit+integration tests,
`cargo test`

For end-to-end:

-   Download <https://github.com/osrf/gazebo_models> as a zip file (cloning doesn't work well)
-   Extract the models to a directory
-   Inside model_processing/crates/webify_models, run cargo run and point it to the gazebo_models extracted directory

Note this is not idempotent, so make sure to keep a copy of the ZIP around if you want to keep re-running it.
