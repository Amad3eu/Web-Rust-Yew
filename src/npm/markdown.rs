// https://github.com/DenisKolodin/yew/blob/master/examples/inner_html/src/lib.rs with marked

// Use NPM or Rust code in util.rs if necessary

import { lexer, parser } from "marked";
import showdown from "showdown";

import { memoizeWith, identity } from "./ramda";

// memoizeWith only for pure function
let html = (input = "") => {
  const test = parser(lexer(input));
  return test;
};

html = memoizeWith(identity, html);

let markdown = (input = "") => {
    const converter = new showdown.Converter();
    const result = converter.makeMarkdown(input);
    return result;
}

markdown = memoizeWith(identity, markdown);