# `sample_dirs`

A little script I made to help me build training/test sets of various sizes for image classification.

Randomly samples the top-level subdirectories of a given directory, and places the results in an output directory.

## Todo

- Clean up `unwraps`
- Delete output directory if it already exists?
- Error handling for when `n` is larger than the number of files in a directory