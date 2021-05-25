module.exports = {
  env: {
    browser: true,
    es2021: true,
  },
  parserOptions: {
    ecmaVersion: 12,
    sourceType: "module",
  },
  rules: {
    "require-jsdoc": 0,
    "max-len": ["error", { code: 140 }],
  },
  extends: ["prettier"],
};
