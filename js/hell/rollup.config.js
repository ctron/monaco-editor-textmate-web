import { nodeResolve } from '@rollup/plugin-node-resolve';
import replace from '@rollup/plugin-replace';
import terser from '@rollup/plugin-terser';
import commonjs from '@rollup/plugin-commonjs';
import builtins from 'rollup-plugin-node-builtins';
import url from '@rollup/plugin-url';

export default {
  input: "main.js",
  output: [
    {
      file: "../debug/sys.js",
      format: "esm",
      compact: false,
    },
    {
      file: "../release/sys.js",
      format: "esm",
      compact: true,
      plugins: [
        terser()
      ]
    }
  ],
  plugins: [
    commonjs(),
    builtins(),
    nodeResolve(),
    url({
      include: ['**/*.wasm'],
      limit: 512 * 1024 * 1024,
    }),
    replace({
      "preventAssignment": true,
      "values": {
        "process.env.NODE_ENV": JSON.stringify('production')
      }
    }),
  ]
}