# elevation-mini

Very fast elevation estimates for any point on Earth.

### Usage

```rust
use elevation_mini::elevation;

fn main() {
    println!("Denver International Airport elevation: {} meters", elevation(39.847426, -104.673957));
}
```

### Details

Uses the [ETOPO1](https://www.ngdc.noaa.gov/mgg/global/relief/ETOPO1/tiled/) "bedrock" dataset from the NOAA, subsampled to 1/10 resolution and interpolates between the closest "known" points.

~50ms per 1 million estimates

No dependencies.
