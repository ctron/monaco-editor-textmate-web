import {INITIAL} from 'monaco-textmate'
import {TMToMonacoToken} from 'monaco-editor-textmate/dist/tm-to-monaco-token';

// the following code comes from `monaco-editor-textmate`

class TokenizerState {
  _ruleStack;

  constructor(_ruleStack) {
    this._ruleStack = _ruleStack;
  }

  get ruleStack() {
    return this._ruleStack;
  }

  clone() {
    return new TokenizerState(this._ruleStack);
  }

  equals(other) {
    if (!other ||
        !(other instanceof TokenizerState) ||
        other !== this ||
        other._ruleStack !== this._ruleStack) {
      return false;
    }
    return true;
  }
}

export function buildProvider(editor, grammar) {
  return {
    getInitialState: () => new TokenizerState(INITIAL),
    tokenize: (line, state) => {
      const res = grammar.tokenizeLine(line, state.ruleStack)
      return {
        endState: new TokenizerState(res.ruleStack),
        tokens: res.tokens.map(token => ({
          ...token,
          // TODO: At the moment, monaco-editor doesn't seem to accept array of scopes
          scopes: editor ? TMToMonacoToken(editor, token.scopes) : token.scopes[token.scopes.length - 1]
        })),
      }
    }
  }
}