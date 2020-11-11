const path = require('path')
const CopyPlugin = require('copy-webpack-plugin')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const MonacoWebpackPlugin = require('monaco-editor-webpack-plugin')
const TerserPlugin = require('terser-webpack-plugin')
const APP_DIR = path.resolve(__dirname, './src')
const MONACO_DIR = path.resolve(__dirname, './node_modules/monaco-editor')

const dist = path.resolve(__dirname, 'dist')

module.exports = {
  mode: 'production',
  entry: {
    index: './js/main.tsx',
  },
  output: {
    path: dist,
    filename: '[name].js',
  },
  devServer: {
    contentBase: dist,
  },
  module: {
    rules: [
      { test: /.tsx?$/, use: 'ts-loader', exclude: /node_modules/ },
      // { test: /.wasm$/, use: 'wasm-loader', exclude: /node_modules/ },
      {
        test: /.css$/,
        include: APP_DIR,
        use: ['style-loader', 'css-loader'],
      },
      {
        test: /.css$/,
        include: MONACO_DIR,
        use: ['style-loader', 'css-loader'],
      },
      { test: /.styl$/, use: ['style-loader', 'css-loader', 'stylus-loader'] },
      { test: /.ttf$/, use: 'file-loader' },
    ],
  },
  plugins: [
    new CopyPlugin([path.resolve(__dirname, 'static')]),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
  optimization: {
    minimize: true,
    minimizer: [new TerserPlugin()],
  },
  experiments: {
    asyncWebAssembly: true,
  },
}
