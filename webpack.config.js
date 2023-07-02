const path = require('path');
const TauriPlugin = require('tauri-webpack-plugin');

module.exports = {
  entry: './src/main.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js',
  },
  plugins: [
    new TauriPlugin({
      cfg: './src-tauri/tauri.conf.json',
    }),
  ],
  module: {
    rules: [
      {
        test: /\.rs$/,
        use: [
          {
            loader: 'rust-loader',
            options: {
              release: true,
            },
          },
        ],
      },
    ],
  },
  resolve: {
    extensions: ['.js', '.rs'],
  },
};