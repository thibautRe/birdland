module.exports = {
  extends: [
    'eslint:recommended',
    'prettier',
  ],
  env: {
    browser: true,
    commonjs: true,
  },
  plugins: [
    'prettier',
  ],
  parser: 'babel-eslint',
  parserOptions: {
    ecmaVersion: 6,
    sourceType: 'module',
  },
  rules: {
    "prettier/prettier": ['error', {
      semi: false,
      singleQuote: true,
      trailingComma: 'all',
    }],
  }
}
