import onigasm from 'onigasm/lib/onigasm.wasm';
import {loadWASM} from 'onigasm';
import {Registry} from 'monaco-textmate';
import {buildProvider} from "./provider";

let initialized = false;

async function checkInit() {
  if (!initialized) {
    initialized = true;
    await loadWASM(onigasm)
  }
}

export class GrammarDefinition {
  constructor(format, content) {
    this.format = format;
    this.content = content;
  }
}

/**
 * Create a registry with a single Grammar
 *
 * @param {GrammarDefinition} grammar The grammar to use
 * @returns {Registry} The new registry instance
 */
export function singleRegistry(grammar) {
  return new Registry({
    getGrammarDefinition: async (scopeName) => {
      return {
        format: grammar.format,
        content: grammar.content,
      }
    }
  });
}

/**
 * Create a new TokensProvider instance.
 *
 * @param editor The monaco code editor instance.
 * @param {GrammarDefinition} grammarDefinition The grammar definition
 * @param {string}initialScope The initial scope of the text mate grammar.
 * @returns {Promise<{getInitialState: function(): TokenizerState, tokenize: function(*, *): {endState: TokenizerState, tokens: *}}>}
 */
export async function createTokensProvider(editor, grammarDefinition, initialScope) {
  // ensure we are initialized
  await checkInit();

  // create the registry
  const registry = await singleRegistry(grammarDefinition);
  // load the grammar
  const g = await registry.loadGrammar(initialScope);

  // build and return the provider
  return buildProvider(editor, g);
}
