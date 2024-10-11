# ilmn2ped
This script converts an input Illumina SNP array Final Report text file, either CSV or TSV, and converts it to [plink](https://www.cog-genomics.org/plink/) [ped/map](https://www.cog-genomics.org/plink/1.9/input#ped) file format.

## Installation
[Rust](https://www.rust-lang.org/) should be installed, with cargo available.
Then, the package can be installed using:
```
git clone https://github.com/RenzoTale88/ilmn2ped.git && \
    pushd ilmn2ped && \
    cargo install --path . && \
    popd
ilmn2ped --help
```

## Options
The script takes one positional argument (the input Final report CSV/TSV file), and two optional options:
1. `--coding`: this can be one of the acceptable Illumina coding formats;
2. `--output`: this is the root name of the output ped/map files.

The acceptable codings are:
1. `top` (default)
2. `bottom`
3. `forward`
4. `reverse`
5. `ab`

The coding has to be present in the input dataset, with the script failing if it won't be able to find it.