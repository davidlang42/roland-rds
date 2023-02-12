# roland-rds
Library/CLI tool for working with Roland RDS files (live set keyboard patches)

## Background
Roland stage pianos (such as the RD300NX) have the abiity to program a set of patches in a particular order for use during a performance, these are called "live sets". The full set of live set patches can be saved/loaded to the internal memory or an external USB drive. They are stored in a binary file with the extension RDS.

Although the functionality in the keyboard is great for performances, due to being an embedded system with limited controls and no keyboard, it can take a very long time to program a complex set of live set patches, and as it is tedious it is prone to human error. More significantly, making small changes such as changing the order of 2 patches, or inserting a single patch and shift everything else down can take hours.

Due to these reasons, a way of editing these RDS files would be very valuable, even if not all parameter values are initially editable. This library is a starting point, which overtime I expect to support:
- editing all parameter values for the live set
- other Roland stage pianos, such as:
- - RD700NX (also uses 2160byte live sets and seemingly the same structure as RD300NX but with more patches)
- - RD-2000 (appears to use 5090byte live sets)
- a GUI application to edit patches (not part of this repo, but perhaps another project someone might like to undertake)

## Usage

To decode an RDS file, and output plain text JSON (which can be easily manipulated in a text editor):

`roland-rds decode FILE.RDS`

If you would like to save the output (rather than see it on the screen):

`roland-rds decode FILE.RDS > FILE.JSON`

If you would like the JSON to be formatted nicely, I recommend chaining with jq:

`roland-rds decode FILE.RDS | jq > FILE.JSON`

Once you have made the required changes to your JSON file, to re-encode it to an RDS file:

`roland-rds encode FILE.JSON`

To save the output of this encoding (to transfer back to the keyboard):

`roland-rds encode FILE.JSON > NEW_FILE.RDS`

To split a decoded JSON file into a folder structure (with each live set as a separate file for easy modification):

`roland-rds split FILE.JSON FOLDER`

To re-combine a folder structure of JSON files into a single JSON file:

`roland-rds merge FOLDER`

If you would like to save the output (rather than see it on the screen):

`roland-rds merge FOLDER > NEW_FILE.JSON`

In the `encode`, `decode` and `split` operations, if you omit the input filename, it will read from std in. The output of `encode`, `decode` and `merge` is always written to std out.

If you run `roland-rds help` or without args, you will see usage instructions.

## Development
I encourage anyone who has time to add to the understanding of the RDS file, or implement for additional Roland devices. I suggest the following 2 approaches, potentially used in tandem.
You can also find some easier (more well defined) tasks in [issues](https://github.com/davidlang42/roland-rds/issues) which will help.

### Generate & diff
- use a keyboard to create 2 different RDS files with only a single parameter different
- decode both files to JSON
- use a text comparing tool to diff the JSON and determine which bit has changed
- update the code in roland.rs accordingly, test, and submit a pull request

### Read implementation
- Roland will likely never release details on the RDS file format
- they do however describe in detail how memory is stored in the device itself as part of the MIDI implementation (see SysEx messsages)
- it appears that the RDS file follows the same parameters but in a compressed format
