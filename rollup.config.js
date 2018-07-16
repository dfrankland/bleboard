import babel from 'rollup-plugin-babel';
import commonjs from 'rollup-plugin-commonjs';
import nodeResolve from 'rollup-plugin-node-resolve';
import { dependencies } from './package.json';

export default {
  input: './src/index.js',
  output: {
    file: './dist/index.js',
    format: 'cjs',
  },
  plugins: [
    babel({
      babelrc: false,
      presets: [
        [
          '@babel/preset-env',
          {
            modules: false,
            targets: {
              node: '8',
            },
          },
        ],
        [
          '@babel/preset-stage-0',
          {
            decoratorsLegacy: true,
            pipelineProposal: 'minimal',
          },
        ],
      ],
    }),
    commonjs(),
    nodeResolve(),
  ],
  external: Object.keys(dependencies),
};
