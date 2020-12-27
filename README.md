# Hebrew_Seed

## Get hebrew text from latin characters

An easy way to use your latin keyboard to type in hebrew script.
Developed with Rust language, using the Seed framework.

## Usage

Refer to the table of correspondence to type the desired chars.

After a time this will become quite natural, and you will no longer need to refer to the tables.

Try live at : https://ivrit-seed-app.netlify.app/ (works with Firefox and Chrome, does not work with Safari which does not accept wasm modules)

## Development

We found our introduction to the Seed framework, in its "pure Rust, no npm package" version, in https://dev.to/arnthelongbeard/how-to-only-rust-for-web-frontend-1026 . Our program closely follows the "counter" seed example.


For comparison's sake, the same type of software was developed:

- using clojurescript : https://github.com/francisstephan/farsiscript

- using elm : https://github.com/francisstephan/arabicscript

- using scalajs : https://github.com/francisstephan/pashtoscript

- using dart : https://github.com/francisstephan/turkishscript

- using Reasonml : https://github.com/francisstephan/hebrewscript

The Seed framework is implemented in lib.rs, which closely follows the elm language model (see the elm version for a comparison).

The "tohebrew.rs" module demonstratezs the possibility to use functional programming with rust.


## License

MIT
